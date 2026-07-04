use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub async fn insert_product(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    sku: &str,
    name: &str,
    price_amount: i64,
    price_currency: &str,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO inventory.products
         (id, tenant_id, sku, name, price_amount, price_currency)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(id)
    .bind(tenant_id.as_uuid())
    .bind(sku)
    .bind(name)
    .bind(price_amount)
    .bind(price_currency)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn list_product_ids(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<Uuid>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_scalar::<_, Uuid>("SELECT id FROM inventory.products ORDER BY sku")
        .fetch_all(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(rows)
}

pub async fn upsert_stock_balance(
    pool: &PgPool,
    tenant_id: TenantId,
    driver_id: Uuid,
    product_id: Uuid,
    quantity: i32,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO inventory.stock_balances (tenant_id, driver_id, product_id, quantity)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (tenant_id, driver_id, product_id)
         DO UPDATE SET quantity = EXCLUDED.quantity, updated_at = now()",
    )
    .bind(tenant_id.as_uuid())
    .bind(driver_id)
    .bind(product_id)
    .bind(quantity)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn get_stock_quantity(
    pool: &PgPool,
    tenant_id: TenantId,
    driver_id: Uuid,
    product_id: Uuid,
) -> Result<Option<i32>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let qty = sqlx::query_scalar::<_, i32>(
        "SELECT quantity FROM inventory.stock_balances
         WHERE tenant_id = $1 AND driver_id = $2 AND product_id = $3",
    )
    .bind(tenant_id.as_uuid())
    .bind(driver_id)
    .bind(product_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(qty)
}
