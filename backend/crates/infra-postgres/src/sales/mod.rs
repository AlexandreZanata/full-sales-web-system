use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::inventory::StockMovementInsert;
use crate::rls::{apply_session_context, apply_tenant_context, SessionContext};

pub struct SaleFilters {
    pub commerce_id: Option<Uuid>,
    pub driver_id: Option<Uuid>,
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    pub to: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct SaleListRow {
    pub id: Uuid,
    pub driver_id: Uuid,
    pub commerce_id: Uuid,
    pub status: String,
    pub payment_method: String,
    pub total_amount: i64,
    pub total_currency: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfirmSaleError {
    #[error("insufficient stock")]
    InsufficientStock,

    #[error("invalid sale transition")]
    InvalidTransition,

    #[error(transparent)]
    Database(#[from] PostgresError),
}

impl From<sqlx::Error> for ConfirmSaleError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(PostgresError::from(err))
    }
}

pub struct ConfirmSaleItem {
    pub product_id: Uuid,
    pub quantity: i32,
}

pub struct NewSaleItem {
    pub id: Uuid,
    pub sale_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price_amount: i64,
    pub unit_price_currency: String,
    pub line_total_amount: i64,
}

pub struct SaleRow {
    pub id: Uuid,
    pub driver_id: Uuid,
    pub commerce_id: Uuid,
    pub order_id: Option<Uuid>,
    pub status: String,
    pub payment_method: String,
    pub declared_payment_method: String,
    pub declared_payment_received: bool,
    pub total_amount: i64,
    pub total_currency: String,
}

pub struct DeclarePaymentUpdate {
    pub sale_id: Uuid,
    pub driver_id: Uuid,
    pub method: String,
    pub received: bool,
    pub declared_at: chrono::DateTime<chrono::Utc>,
    pub notes: Option<String>,
}

pub struct SaleItemRow {
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price_amount: i64,
    pub unit_price_currency: String,
    pub line_total_amount: i64,
}

pub struct SaleInsert {
    pub sale_id: Uuid,
    pub driver_id: Uuid,
    pub commerce_id: Uuid,
    pub payment_method: String,
    pub total_amount: i64,
    pub total_currency: String,
    pub items: Vec<NewSaleItem>,
}

pub async fn insert_sale_with_items(
    pool: &PgPool,
    tenant_id: TenantId,
    sale: SaleInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO sales.sales
         (id, tenant_id, driver_id, commerce_id, payment_method, total_amount, total_currency)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(sale.sale_id)
    .bind(tenant_id.as_uuid())
    .bind(sale.driver_id)
    .bind(sale.commerce_id)
    .bind(&sale.payment_method)
    .bind(sale.total_amount)
    .bind(&sale.total_currency)
    .execute(&mut *tx)
    .await?;

    for item in &sale.items {
        sqlx::query(
            "INSERT INTO sales.sale_items
             (id, tenant_id, sale_id, product_id, quantity, unit_price_amount, unit_price_currency, line_total_amount)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(item.id)
        .bind(tenant_id.as_uuid())
        .bind(item.sale_id)
        .bind(item.product_id)
        .bind(item.quantity)
        .bind(item.unit_price_amount)
        .bind(&item.unit_price_currency)
        .bind(item.line_total_amount)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn find_sale_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<SaleRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (
        Uuid,
        Uuid,
        Uuid,
        Option<Uuid>,
        String,
        String,
        String,
        bool,
        i64,
        String,
    )>(
        "SELECT id, driver_id, commerce_id, order_id, status, payment_method,
                declared_payment_method, declared_payment_received, total_amount, total_currency
         FROM sales.sales WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(
        |(
            id,
            driver_id,
            commerce_id,
            order_id,
            status,
            payment_method,
            declared_payment_method,
            declared_payment_received,
            total_amount,
            total_currency,
        )| {
            SaleRow {
                id,
                driver_id,
                commerce_id,
                order_id,
                status,
                payment_method,
                declared_payment_method,
                declared_payment_received,
                total_amount,
                total_currency,
            }
        },
    ))
}

pub async fn list_sale_items(
    pool: &PgPool,
    tenant_id: TenantId,
    sale_id: Uuid,
) -> Result<Vec<SaleItemRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (Uuid, i32, i64, String, i64)>(
        "SELECT product_id, quantity, unit_price_amount, unit_price_currency, line_total_amount
         FROM sales.sale_items WHERE sale_id = $1 ORDER BY created_at",
    )
    .bind(sale_id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(product_id, quantity, unit_price_amount, unit_price_currency, line_total_amount)| {
                SaleItemRow {
                    product_id,
                    quantity,
                    unit_price_amount,
                    unit_price_currency,
                    line_total_amount,
                }
            },
        )
        .collect())
}

pub async fn confirm_sale_status(
    pool: &PgPool,
    tenant_id: TenantId,
    sale_id: Uuid,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE sales.sales SET status = 'Confirmed', confirmed_at = now()
         WHERE id = $1 AND status = 'Pending'",
    )
    .bind(sale_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

/// BR-IN-002: deduct stock, record SaleOutbound movements, and confirm sale atomically.
pub async fn confirm_sale_with_stock(
    pool: &PgPool,
    tenant_id: TenantId,
    driver_id: Uuid,
    sale_id: Uuid,
    items: &[ConfirmSaleItem],
) -> Result<(), ConfirmSaleError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;

    for item in items {
        let result = sqlx::query(
            "UPDATE inventory.stock_balances
             SET quantity = quantity - $4, updated_at = now()
             WHERE tenant_id = $1 AND driver_id = $2 AND product_id = $3 AND quantity >= $4",
        )
        .bind(tenant_id.as_uuid())
        .bind(driver_id)
        .bind(item.product_id)
        .bind(item.quantity)
        .execute(&mut *tx)
        .await?;
        if result.rows_affected() != 1 {
            return Err(ConfirmSaleError::InsufficientStock);
        }

        let movement = StockMovementInsert {
            id: Uuid::now_v7(),
            product_id: item.product_id,
            responsible_id: driver_id,
            movement_type: "SaleOutbound".to_owned(),
            quantity: item.quantity,
            reference_id: Some(sale_id),
            reason: None,
        };
        sqlx::query(
            "INSERT INTO inventory.stock_movements
             (id, tenant_id, product_id, responsible_id, movement_type, quantity, reference_id, reason)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(movement.id)
        .bind(tenant_id.as_uuid())
        .bind(movement.product_id)
        .bind(movement.responsible_id)
        .bind(movement.movement_type)
        .bind(movement.quantity)
        .bind(movement.reference_id)
        .bind(movement.reason)
        .execute(&mut *tx)
        .await?;
    }

    let status = sqlx::query(
        "UPDATE sales.sales SET status = 'Confirmed', confirmed_at = now()
         WHERE id = $1 AND status = 'Pending'",
    )
    .bind(sale_id)
    .execute(&mut *tx)
    .await?;
    if status.rows_affected() != 1 {
        return Err(ConfirmSaleError::InvalidTransition);
    }

    tx.commit().await?;
    Ok(())
}

pub async fn cancel_sale_status(
    pool: &PgPool,
    tenant_id: TenantId,
    sale_id: Uuid,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE sales.sales SET status = 'Cancelled', cancelled_at = now()
         WHERE id = $1 AND status = 'Pending'",
    )
    .bind(sale_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn insert_sale(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    driver_id: Uuid,
    commerce_id: Uuid,
    payment_method: &str,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO sales.sales (id, tenant_id, driver_id, commerce_id, payment_method)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(id)
    .bind(tenant_id.as_uuid())
    .bind(driver_id)
    .bind(commerce_id)
    .bind(payment_method)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn list_sale_ids(pool: &PgPool, tenant_id: TenantId) -> Result<Vec<Uuid>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_scalar::<_, Uuid>("SELECT id FROM sales.sales ORDER BY created_at")
        .fetch_all(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(rows)
}

pub async fn list_sales(
    pool: &PgPool,
    tenant_id: TenantId,
    filters: &SaleFilters,
    limit: i64,
    offset: i64,
) -> Result<Vec<SaleListRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (
        Uuid,
        Uuid,
        Uuid,
        String,
        String,
        i64,
        String,
        chrono::DateTime<chrono::Utc>,
    )>(
        "SELECT id, driver_id, commerce_id, status, payment_method, total_amount, total_currency, created_at
         FROM sales.sales
         WHERE ($1::uuid IS NULL OR commerce_id = $1)
           AND ($2::uuid IS NULL OR driver_id = $2)
           AND ($3::timestamptz IS NULL OR created_at >= $3)
           AND ($4::timestamptz IS NULL OR created_at <= $4)
         ORDER BY created_at DESC
         LIMIT $5 OFFSET $6",
    )
    .bind(filters.commerce_id)
    .bind(filters.driver_id)
    .bind(filters.from)
    .bind(filters.to)
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(
                id,
                driver_id,
                commerce_id,
                status,
                payment_method,
                total_amount,
                total_currency,
                created_at,
            )| SaleListRow {
                id,
                driver_id,
                commerce_id,
                status,
                payment_method,
                total_amount,
                total_currency,
                created_at,
            },
        )
        .collect())
}

pub async fn count_sales(
    pool: &PgPool,
    tenant_id: TenantId,
    filters: &SaleFilters,
) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)
         FROM sales.sales
         WHERE ($1::uuid IS NULL OR commerce_id = $1)
           AND ($2::uuid IS NULL OR driver_id = $2)
           AND ($3::timestamptz IS NULL OR created_at >= $3)
           AND ($4::timestamptz IS NULL OR created_at <= $4)",
    )
    .bind(filters.commerce_id)
    .bind(filters.driver_id)
    .bind(filters.from)
    .bind(filters.to)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(count)
}

/// RN-PAG3 — persist declaration and append audit event in one transaction.
pub async fn declare_payment(
    pool: &PgPool,
    session: &SessionContext,
    update: &DeclarePaymentUpdate,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;

    let previous = sqlx::query_as::<_, (String, bool)>(
        "SELECT declared_payment_method, declared_payment_received
         FROM sales.sales WHERE id = $1 AND driver_id = $2 FOR UPDATE",
    )
    .bind(update.sale_id)
    .bind(update.driver_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(PostgresError::Database(sqlx::Error::RowNotFound))?;

    let result = sqlx::query(
        "UPDATE sales.sales
         SET declared_payment_method = $3,
             declared_payment_received = $4,
             declared_payment_at = $5,
             declared_payment_by_user_id = $6,
             declared_payment_notes = $7
         WHERE id = $1 AND driver_id = $2",
    )
    .bind(update.sale_id)
    .bind(update.driver_id)
    .bind(&update.method)
    .bind(update.received)
    .bind(update.declared_at)
    .bind(session.user_id)
    .bind(&update.notes)
    .execute(&mut *tx)
    .await?;
    if result.rows_affected() != 1 {
        return Err(PostgresError::Database(sqlx::Error::RowNotFound));
    }

    sqlx::query(
        "INSERT INTO audit.events
         (id, tenant_id, actor_id, action, resource_type, resource_id, metadata, correlation_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(Uuid::now_v7())
    .bind(session.tenant_id.as_uuid())
    .bind(session.user_id)
    .bind("sale.declared_payment_changed")
    .bind("Sale")
    .bind(update.sale_id)
    .bind(serde_json::json!({
        "previousMethod": previous.0,
        "previousReceived": previous.1,
        "newMethod": update.method,
        "newReceived": update.received,
    }))
    .bind(None::<Uuid>)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn insert_sale_item(
    pool: &PgPool,
    tenant_id: TenantId,
    item: NewSaleItem,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO sales.sale_items
         (id, tenant_id, sale_id, product_id, quantity, unit_price_amount, unit_price_currency, line_total_amount)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(item.id)
    .bind(tenant_id.as_uuid())
    .bind(item.sale_id)
    .bind(item.product_id)
    .bind(item.quantity)
    .bind(item.unit_price_amount)
    .bind(&item.unit_price_currency)
    .bind(item.line_total_amount)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}
