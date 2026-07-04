use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

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
    pub status: String,
    pub payment_method: String,
    pub total_amount: i64,
    pub total_currency: String,
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
    let row = sqlx::query_as::<_, (Uuid, Uuid, Uuid, String, String, i64, String)>(
        "SELECT id, driver_id, commerce_id, status, payment_method, total_amount, total_currency
         FROM sales.sales WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(
        |(id, driver_id, commerce_id, status, payment_method, total_amount, total_currency)| {
            SaleRow {
                id,
                driver_id,
                commerce_id,
                status,
                payment_method,
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
