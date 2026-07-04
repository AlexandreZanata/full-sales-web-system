use domain_shared::TenantId;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::PostgresError;
use crate::inventory::reservations::{self, ReservationLine};
use crate::rls::{apply_session_context, SessionContext};

pub struct OrderInsert {
    pub id: Uuid,
    pub commerce_id: Uuid,
    pub created_by_user_id: Uuid,
    pub source: String,
    pub status: String,
    pub delivery_address_id: Uuid,
    pub notes: Option<String>,
    pub total_amount: i64,
    pub total_currency: String,
}

pub struct OrderItemInsert {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity_requested: i32,
    pub unit_price_amount: i64,
    pub unit_price_currency: String,
    pub line_total_amount: i64,
}

pub struct OrderRow {
    pub id: Uuid,
    pub commerce_id: Uuid,
    pub created_by_user_id: Uuid,
    pub status: String,
}

pub struct OrderDetailRow {
    pub id: Uuid,
    pub commerce_id: Uuid,
    pub created_by_user_id: Uuid,
    pub status: String,
    pub delivery_address_id: Uuid,
    pub notes: Option<String>,
    pub total_amount: i64,
    pub total_currency: String,
    pub rejection_reason: Option<String>,
    pub source: String,
}

pub struct OrderListRow {
    pub id: Uuid,
    pub status: String,
    pub total_amount: i64,
    pub total_currency: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct OrderItemRow {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity_requested: i32,
    pub unit_price_amount: i64,
    pub unit_price_currency: String,
    pub line_total_amount: i64,
}

pub async fn insert_order(
    pool: &PgPool,
    session: &SessionContext,
    order: &OrderInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    insert_order_in_tx(&mut tx, session.tenant_id, order).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn insert_order_items(
    pool: &PgPool,
    session: &SessionContext,
    items: &[OrderItemInsert],
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    for item in items {
        insert_order_item_in_tx(&mut tx, session.tenant_id, item).await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn find_order_by_id(
    pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
) -> Result<Option<OrderRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let row = sqlx::query_as::<_, (Uuid, Uuid, Uuid, String)>(
        "SELECT id, commerce_id, created_by_user_id, status
         FROM orders.orders
         WHERE id = $1",
    )
    .bind(order_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(|(id, commerce_id, created_by_user_id, status)| OrderRow {
        id,
        commerce_id,
        created_by_user_id,
        status,
    }))
}

/// Single transaction: PendingApproval → Approved + stock reservations (RN2).
pub async fn approve_order_transaction(
    pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
    reservation_lines: &[ReservationLine],
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;

    let updated = sqlx::query(
        "UPDATE orders.orders
         SET status = 'Approved'
         WHERE id = $1 AND status = 'PendingApproval'",
    )
    .bind(order_id)
    .execute(&mut *tx)
    .await?;

    if updated.rows_affected() == 0 {
        return Err(PostgresError::Database(sqlx::Error::RowNotFound));
    }

    reservations::reserve_stock_in_tx(&mut tx, session.tenant_id.as_uuid(), reservation_lines).await?;
    tx.commit().await?;
    Ok(())
}

/// Single transaction: cancel order and optionally release reservations (RN6).
pub async fn cancel_order_transaction(
    pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
    release_reservations: bool,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;

    let updated = sqlx::query(
        "UPDATE orders.orders
         SET status = 'Cancelled', cancelled_at = now()
         WHERE id = $1
           AND status IN ('Draft', 'PendingApproval', 'Approved', 'Picking')",
    )
    .bind(order_id)
    .execute(&mut *tx)
    .await?;

    if updated.rows_affected() == 0 {
        return Err(PostgresError::Database(sqlx::Error::RowNotFound));
    }

    if release_reservations {
        reservations::release_reservations_in_tx(&mut tx, order_id).await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn update_order_status_in_tx(
    tx: &mut Transaction<'_, Postgres>,
    order_id: Uuid,
    status: &str,
) -> Result<(), PostgresError> {
    let result = sqlx::query("UPDATE orders.orders SET status = $2 WHERE id = $1")
        .bind(order_id)
        .bind(status)
        .execute(&mut **tx)
        .await?;
    if result.rows_affected() == 0 {
        return Err(PostgresError::Database(sqlx::Error::RowNotFound));
    }
    Ok(())
}

pub async fn update_order_status(
    pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
    status: &str,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    update_order_status_in_tx(&mut tx, order_id, status).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn list_orders(
    pool: &PgPool,
    session: &SessionContext,
    status: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<OrderListRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let rows = sqlx::query_as::<_, (Uuid, String, i64, String, chrono::DateTime<chrono::Utc>)>(
        "SELECT id, status, total_amount, total_currency, created_at
         FROM orders.orders
         WHERE ($1::text IS NULL OR status = $1)
         ORDER BY created_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(status)
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(|(id, status, total_amount, total_currency, created_at)| OrderListRow {
            id,
            status,
            total_amount,
            total_currency,
            created_at,
        })
        .collect())
}

pub async fn count_orders(
    pool: &PgPool,
    session: &SessionContext,
    status: Option<&str>,
) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM orders.orders WHERE ($1::text IS NULL OR status = $1)",
    )
    .bind(status)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(count)
}

pub async fn find_order_detail(
    pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
) -> Result<Option<OrderDetailRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let row = sqlx::query_as::<_, (
        Uuid,
        Uuid,
        Uuid,
        String,
        Uuid,
        Option<String>,
        i64,
        String,
        Option<String>,
        String,
    )>(
        "SELECT id, commerce_id, created_by_user_id, status, delivery_address_id, notes,
                total_amount, total_currency, rejection_reason, source
         FROM orders.orders WHERE id = $1",
    )
    .bind(order_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(
        |(
            id,
            commerce_id,
            created_by_user_id,
            status,
            delivery_address_id,
            notes,
            total_amount,
            total_currency,
            rejection_reason,
            source,
        )| OrderDetailRow {
            id,
            commerce_id,
            created_by_user_id,
            status,
            delivery_address_id,
            notes,
            total_amount,
            total_currency,
            rejection_reason,
            source,
        },
    ))
}

pub async fn list_order_items(
    pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
) -> Result<Vec<OrderItemRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let rows = sqlx::query_as::<_, (Uuid, Uuid, i32, i64, String, i64)>(
        "SELECT id, product_id, quantity_requested, unit_price_amount, unit_price_currency, line_total_amount
         FROM orders.order_items WHERE order_id = $1 ORDER BY created_at",
    )
    .bind(order_id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(id, product_id, quantity_requested, unit_price_amount, unit_price_currency, line_total_amount)| {
                OrderItemRow {
                    id,
                    product_id,
                    quantity_requested,
                    unit_price_amount,
                    unit_price_currency,
                    line_total_amount,
                }
            },
        )
        .collect())
}

pub async fn insert_order_with_items(
    pool: &PgPool,
    session: &SessionContext,
    order: &OrderInsert,
    items: &[OrderItemInsert],
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    insert_order_in_tx(&mut tx, session.tenant_id, order).await?;
    for item in items {
        insert_order_item_in_tx(&mut tx, session.tenant_id, item).await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn replace_draft_order(
    pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
    delivery_address_id: Uuid,
    notes: Option<String>,
    total_amount: i64,
    total_currency: &str,
    items: &[OrderItemInsert],
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let updated = sqlx::query(
        "UPDATE orders.orders
         SET delivery_address_id = $2, notes = $3, total_amount = $4, total_currency = $5
         WHERE id = $1 AND status = 'Draft'",
    )
    .bind(order_id)
    .bind(delivery_address_id)
    .bind(notes)
    .bind(total_amount)
    .bind(total_currency)
    .execute(&mut *tx)
    .await?;
    if updated.rows_affected() == 0 {
        return Err(PostgresError::Database(sqlx::Error::RowNotFound));
    }
    sqlx::query("DELETE FROM orders.order_items WHERE order_id = $1")
        .bind(order_id)
        .execute(&mut *tx)
        .await?;
    for item in items {
        insert_order_item_in_tx(&mut tx, session.tenant_id, item).await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn submit_order(
    pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let updated = sqlx::query(
        "UPDATE orders.orders SET status = 'PendingApproval'
         WHERE id = $1 AND status = 'Draft'
           AND EXISTS (SELECT 1 FROM orders.order_items WHERE order_id = $1)",
    )
    .bind(order_id)
    .execute(&mut *tx)
    .await?;
    if updated.rows_affected() == 0 {
        return Err(PostgresError::Database(sqlx::Error::RowNotFound));
    }
    tx.commit().await?;
    Ok(())
}

pub async fn reject_order(
    pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
    reason: &str,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    let updated = sqlx::query(
        "UPDATE orders.orders
         SET status = 'Rejected', rejection_reason = $2
         WHERE id = $1 AND status = 'PendingApproval'",
    )
    .bind(order_id)
    .bind(reason)
    .execute(&mut *tx)
    .await?;
    if updated.rows_affected() == 0 {
        return Err(PostgresError::Database(sqlx::Error::RowNotFound));
    }
    tx.commit().await?;
    Ok(())
}

pub async fn cancel_draft_order(
    pool: &PgPool,
    session: &SessionContext,
    order_id: Uuid,
) -> Result<(), PostgresError> {
    cancel_order_transaction(pool, session, order_id, false).await
}

async fn insert_order_in_tx(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
    order: &OrderInsert,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO orders.orders
         (id, tenant_id, commerce_id, created_by_user_id, source, status,
          delivery_address_id, notes, total_amount, total_currency)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    .bind(order.id)
    .bind(tenant_id.as_uuid())
    .bind(order.commerce_id)
    .bind(order.created_by_user_id)
    .bind(&order.source)
    .bind(&order.status)
    .bind(order.delivery_address_id)
    .bind(&order.notes)
    .bind(order.total_amount)
    .bind(&order.total_currency)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn insert_order_item_in_tx(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
    item: &OrderItemInsert,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO orders.order_items
         (id, tenant_id, order_id, product_id, quantity_requested,
          unit_price_amount, unit_price_currency, line_total_amount)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(item.id)
    .bind(tenant_id.as_uuid())
    .bind(item.order_id)
    .bind(item.product_id)
    .bind(item.quantity_requested)
    .bind(item.unit_price_amount)
    .bind(&item.unit_price_currency)
    .bind(item.line_total_amount)
    .execute(&mut **tx)
    .await?;
    Ok(())
}
