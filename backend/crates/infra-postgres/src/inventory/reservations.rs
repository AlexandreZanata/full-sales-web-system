use domain_shared::TenantId;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct ReservationLine {
    pub id: Uuid,
    pub order_id: Uuid,
    pub order_item_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub driver_id: Option<Uuid>,
}

async fn lock_product(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: Uuid,
    product_id: Uuid,
) -> Result<(), PostgresError> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1::text || $2::text))")
        .bind(tenant_id.to_string())
        .bind(product_id.to_string())
        .execute(&mut **tx)
        .await?;
    Ok(())
}

async fn tenant_product_available(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: Uuid,
    product_id: Uuid,
) -> Result<i32, PostgresError> {
    let balance: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(quantity), 0)
         FROM inventory.stock_balances
         WHERE tenant_id = $1 AND product_id = $2",
    )
    .bind(tenant_id)
    .bind(product_id)
    .fetch_one(&mut **tx)
    .await?;

    let reserved: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(quantity_reserved), 0)
         FROM inventory.stock_reservations
         WHERE tenant_id = $1 AND product_id = $2 AND status = 'Active'",
    )
    .bind(tenant_id)
    .bind(product_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok((balance - reserved).max(0) as i32)
}

/// Reserves stock for order approval (RN2). Uses advisory locks to prevent oversell.
pub async fn reserve_stock_for_order(
    pool: &PgPool,
    tenant_id: TenantId,
    lines: &[ReservationLine],
) -> Result<(), PostgresError> {
    if lines.is_empty() {
        return Ok(());
    }

    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let tenant_uuid = tenant_id.as_uuid();

    let mut demand: std::collections::HashMap<Uuid, i32> = std::collections::HashMap::new();
    for line in lines {
        *demand.entry(line.product_id).or_insert(0) += line.quantity;
    }

    for (product_id, qty) in &demand {
        lock_product(&mut tx, tenant_uuid, *product_id).await?;
        let available = tenant_product_available(&mut tx, tenant_uuid, *product_id).await?;
        if available < *qty {
            return Err(PostgresError::InsufficientAvailableStock);
        }
    }

    for line in lines {
        sqlx::query(
            "INSERT INTO inventory.stock_reservations
             (id, tenant_id, order_id, order_item_id, product_id, driver_id, quantity_reserved, status)
             VALUES ($1, $2, $3, $4, $5, $6, $7, 'Active')",
        )
        .bind(line.id)
        .bind(tenant_uuid)
        .bind(line.order_id)
        .bind(line.order_item_id)
        .bind(line.product_id)
        .bind(line.driver_id)
        .bind(line.quantity)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

/// RN6 — releases Active reservations when an order is cancelled before InTransit.
pub async fn release_reservations(
    pool: &PgPool,
    tenant_id: TenantId,
    order_id: Uuid,
) -> Result<u64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE inventory.stock_reservations
         SET status = 'Released', released_at = now()
         WHERE order_id = $1 AND status = 'Active'",
    )
    .bind(order_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected())
}

/// RN2 — marks Active reservations Consumed on delivery confirm (deduction in Phase 12–13).
pub async fn consume_reservations(
    pool: &PgPool,
    tenant_id: TenantId,
    order_id: Uuid,
) -> Result<u64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE inventory.stock_reservations
         SET status = 'Consumed', consumed_at = now()
         WHERE order_id = $1 AND status = 'Active'",
    )
    .bind(order_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected())
}

pub async fn tenant_available_for_product(
    pool: &PgPool,
    tenant_id: TenantId,
    product_id: Uuid,
) -> Result<i32, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let available =
        tenant_product_available(&mut tx, tenant_id.as_uuid(), product_id).await?;
    tx.commit().await?;
    Ok(available)
}

pub struct ReservationRow {
    pub id: Uuid,
    pub order_id: Uuid,
    pub order_item_id: Uuid,
    pub product_id: Uuid,
    pub quantity_reserved: i32,
    pub status: String,
}

pub async fn list_reservations_by_order(
    pool: &PgPool,
    tenant_id: TenantId,
    order_id: Uuid,
) -> Result<Vec<ReservationRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (Uuid, Uuid, Uuid, Uuid, i32, String)>(
        "SELECT id, order_id, order_item_id, product_id, quantity_reserved, status
         FROM inventory.stock_reservations
         WHERE order_id = $1
         ORDER BY created_at",
    )
    .bind(order_id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(id, order_id, order_item_id, product_id, quantity_reserved, status)| {
                ReservationRow {
                    id,
                    order_id,
                    order_item_id,
                    product_id,
                    quantity_reserved,
                    status,
                }
            },
        )
        .collect())
}
