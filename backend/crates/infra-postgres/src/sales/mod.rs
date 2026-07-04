use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct NewSaleItem<'a> {
    pub id: Uuid,
    pub sale_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price_amount: i64,
    pub unit_price_currency: &'a str,
    pub line_total_amount: i64,
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
    item: NewSaleItem<'_>,
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
    .bind(item.unit_price_currency)
    .bind(item.line_total_amount)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}
