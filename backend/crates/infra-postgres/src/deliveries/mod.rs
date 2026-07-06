use domain_shared::TenantId;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::PostgresError;
use crate::inventory::StockMovementInsert;
use crate::inventory::reservations;
use crate::orders;
use crate::rls::{SessionContext, apply_session_context};
use crate::sales;

pub struct DeliveryInsert {
    pub id: Uuid,
    pub order_id: Uuid,
    pub driver_id: Uuid,
    pub status: String,
}

pub struct DeliveryRow {
    pub id: Uuid,
    pub order_id: Uuid,
    pub driver_id: Uuid,
    pub status: String,
}

pub struct ConfirmDeliveryItemUpdate {
    pub order_item_id: Uuid,
    pub quantity_delivered: i32,
}

pub struct ConfirmDeliveryTxInput {
    pub delivery_id: Uuid,
    pub order_id: Uuid,
    pub driver_id: Uuid,
    pub order_status: String,
    pub proof_file_id: Uuid,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub received_by_name: Option<String>,
    pub order_items: Vec<ConfirmDeliveryItemUpdate>,
    pub sale_id: Uuid,
    pub commerce_id: Uuid,
    pub sale_items: Vec<sales::NewSaleItem>,
    pub stock_lines: Vec<sales::ConfirmSaleItem>,
}

pub async fn insert_delivery(
    pool: &PgPool,
    session: &SessionContext,
    row: &DeliveryInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    insert_delivery_in_tx(&mut tx, session.tenant_id, row).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn find_delivery_by_id(
    pool: &PgPool,
    session: &SessionContext,
    delivery_id: Uuid,
) -> Result<Option<DeliveryRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let row = sqlx::query_as::<_, (Uuid, Uuid, Uuid, String)>(
        "SELECT id, order_id, driver_id, status FROM deliveries.deliveries WHERE id = $1",
    )
    .bind(delivery_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(|(id, order_id, driver_id, status)| DeliveryRow {
        id,
        order_id,
        driver_id,
        status,
    }))
}

pub struct DeliveryFilters {
    pub driver_id: Option<Uuid>,
    pub status: Option<String>,
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    pub to: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn list_deliveries(
    pool: &PgPool,
    session: &SessionContext,
    filters: &DeliveryFilters,
    limit: i64,
    offset: i64,
) -> Result<Vec<DeliveryRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let rows = sqlx::query_as::<_, (Uuid, Uuid, Uuid, String)>(
        "SELECT id, order_id, driver_id, status FROM deliveries.deliveries
         WHERE ($1::uuid IS NULL OR driver_id = $1)
           AND ($2::text IS NULL OR status = $2)
         ORDER BY created_at DESC
         LIMIT $3 OFFSET $4",
    )
    .bind(filters.driver_id)
    .bind(filters.status.as_deref())
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(|(id, order_id, driver_id, status)| DeliveryRow {
            id,
            order_id,
            driver_id,
            status,
        })
        .collect())
}

pub async fn list_deliveries_cursor(
    pool: &PgPool,
    session: &SessionContext,
    filters: &DeliveryFilters,
    before_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<DeliveryRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let rows = sqlx::query_as::<_, (Uuid, Uuid, Uuid, String)>(
        "SELECT id, order_id, driver_id, status FROM deliveries.deliveries
         WHERE ($1::uuid IS NULL OR driver_id = $1)
           AND ($2::text IS NULL OR status = $2)
           AND ($3::timestamptz IS NULL OR created_at >= $3)
           AND ($4::timestamptz IS NULL OR created_at <= $4)
           AND ($5::uuid IS NULL OR id < $5)
         ORDER BY id DESC
         LIMIT $6",
    )
    .bind(filters.driver_id)
    .bind(filters.status.as_deref())
    .bind(filters.from)
    .bind(filters.to)
    .bind(before_id)
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(|(id, order_id, driver_id, status)| DeliveryRow {
            id,
            order_id,
            driver_id,
            status,
        })
        .collect())
}

pub async fn count_deliveries(
    pool: &PgPool,
    session: &SessionContext,
    filters: &DeliveryFilters,
) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM deliveries.deliveries
         WHERE ($1::uuid IS NULL OR driver_id = $1)
           AND ($2::text IS NULL OR status = $2)",
    )
    .bind(filters.driver_id)
    .bind(filters.status.as_deref())
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(count)
}

pub async fn find_delivery_by_order_id(
    pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
) -> Result<Option<DeliveryRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let row = sqlx::query_as::<_, (Uuid, Uuid, Uuid, String)>(
        "SELECT id, order_id, driver_id, status FROM deliveries.deliveries WHERE order_id = $1",
    )
    .bind(order_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(|(id, order_id, driver_id, status)| DeliveryRow {
        id,
        order_id,
        driver_id,
        status,
    }))
}

/// Phase 12 preview — atomic confirm: delivery (driver RLS), then order/sale/stock (admin).
pub async fn confirm_delivery_transaction(
    pool: &PgPool,
    driver_session: &SessionContext,
    admin_session: &SessionContext,
    input: &ConfirmDeliveryTxInput,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, driver_session).await?;

    let delivery_updated = sqlx::query(
        "UPDATE deliveries.deliveries
         SET status = 'Delivered',
             proof_file_id = $2,
             delivery_latitude = $3,
             delivery_longitude = $4,
             received_by_name = $5,
             delivered_at = now()
         WHERE id = $1 AND status = 'InTransit' AND driver_id = $6",
    )
    .bind(input.delivery_id)
    .bind(input.proof_file_id)
    .bind(input.latitude)
    .bind(input.longitude)
    .bind(&input.received_by_name)
    .bind(input.driver_id)
    .execute(&mut *tx)
    .await?;
    if delivery_updated.rows_affected() == 0 {
        return Err(PostgresError::Database(sqlx::Error::RowNotFound));
    }

    apply_session_context(&mut tx, admin_session).await?;
    let tenant_uuid = admin_session.tenant_id.as_uuid();

    let order_updated =
        sqlx::query("UPDATE orders.orders SET status = $2 WHERE id = $1 AND status = 'InTransit'")
            .bind(input.order_id)
            .bind(&input.order_status)
            .execute(&mut *tx)
            .await?;
    if order_updated.rows_affected() == 0 {
        return Err(PostgresError::Database(sqlx::Error::RowNotFound));
    }

    for item in &input.order_items {
        sqlx::query(
            "UPDATE orders.order_items
             SET quantity_delivered = $3
             WHERE id = $1 AND order_id = $2",
        )
        .bind(item.order_item_id)
        .bind(input.order_id)
        .bind(item.quantity_delivered)
        .execute(&mut *tx)
        .await?;
    }

    let total_amount: i64 = input
        .sale_items
        .iter()
        .map(|item| item.line_total_amount)
        .sum();

    sqlx::query(
        "INSERT INTO sales.sales
         (id, tenant_id, driver_id, commerce_id, order_id, payment_method, status, total_amount, total_currency, confirmed_at)
         VALUES ($1, $2, $3, $4, $5, 'NotDeclared', 'Confirmed', $6, 'BRL', now())",
    )
    .bind(input.sale_id)
    .bind(tenant_uuid)
    .bind(input.driver_id)
    .bind(input.commerce_id)
    .bind(input.order_id)
    .bind(total_amount)
    .execute(&mut *tx)
    .await?;

    for item in &input.sale_items {
        sqlx::query(
            "INSERT INTO sales.sale_items
             (id, tenant_id, sale_id, product_id, quantity, unit_price_amount, unit_price_currency, line_total_amount)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(item.id)
        .bind(tenant_uuid)
        .bind(item.sale_id)
        .bind(item.product_id)
        .bind(item.quantity)
        .bind(item.unit_price_amount)
        .bind(&item.unit_price_currency)
        .bind(item.line_total_amount)
        .execute(&mut *tx)
        .await?;
    }

    let metrics: Vec<(Uuid, i32)> = input
        .sale_items
        .iter()
        .map(|item| (item.product_id, item.quantity))
        .collect();
    crate::sales::record_product_sales_in_tx(&mut tx, tenant_uuid, &metrics).await?;

    for line in &input.stock_lines {
        let result = sqlx::query(
            "UPDATE inventory.stock_balances
             SET quantity = quantity - $4, updated_at = now()
             WHERE tenant_id = $1 AND driver_id = $2 AND product_id = $3 AND quantity >= $4",
        )
        .bind(tenant_uuid)
        .bind(input.driver_id)
        .bind(line.product_id)
        .bind(line.quantity)
        .execute(&mut *tx)
        .await?;
        if result.rows_affected() != 1 {
            return Err(PostgresError::InsufficientAvailableStock);
        }

        let movement = StockMovementInsert {
            id: Uuid::now_v7(),
            product_id: line.product_id,
            responsible_id: input.driver_id,
            movement_type: "SaleOutbound".to_owned(),
            quantity: line.quantity,
            reference_id: Some(input.sale_id),
            reason: None,
        };
        sqlx::query(
            "INSERT INTO inventory.stock_movements
             (id, tenant_id, product_id, responsible_id, movement_type, quantity, reference_id, reason)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(movement.id)
        .bind(tenant_uuid)
        .bind(movement.product_id)
        .bind(movement.responsible_id)
        .bind(movement.movement_type)
        .bind(movement.quantity)
        .bind(movement.reference_id)
        .bind(movement.reason)
        .execute(&mut *tx)
        .await?;
    }

    reservations::consume_reservations_in_tx(&mut tx, input.order_id).await?;
    tx.commit().await?;
    Ok(())
}

async fn insert_delivery_in_tx(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
    row: &DeliveryInsert,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO deliveries.deliveries (id, tenant_id, order_id, driver_id, status)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(row.id)
    .bind(tenant_id.as_uuid())
    .bind(row.order_id)
    .bind(row.driver_id)
    .bind(&row.status)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn start_transit_in_tx(
    tx: &mut Transaction<'_, Postgres>,
    delivery_id: Uuid,
    driver_id: Uuid,
) -> Result<(), PostgresError> {
    let result = sqlx::query(
        "UPDATE deliveries.deliveries
         SET status = 'InTransit'
         WHERE id = $1 AND driver_id = $2 AND status = 'Waiting'",
    )
    .bind(delivery_id)
    .bind(driver_id)
    .execute(&mut **tx)
    .await?;
    if result.rows_affected() == 0 {
        return Err(PostgresError::Database(sqlx::Error::RowNotFound));
    }
    Ok(())
}

pub async fn start_delivery_transit(
    pool: &PgPool,
    driver_session: &SessionContext,
    admin_session: &SessionContext,
    delivery_id: Uuid,
    driver_id: Uuid,
    order_id: Uuid,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, driver_session).await?;
    start_transit_in_tx(&mut tx, delivery_id, driver_id).await?;
    apply_session_context(&mut tx, admin_session).await?;
    orders::update_order_status_in_tx(&mut tx, order_id, "InTransit").await?;
    tx.commit().await?;
    Ok(())
}
