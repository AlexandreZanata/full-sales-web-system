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

pub struct ProductRow {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub price_amount: i64,
    pub price_currency: String,
    pub active: bool,
}

pub async fn list_products(
    pool: &PgPool,
    tenant_id: TenantId,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProductRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (Uuid, String, String, i64, String, bool)>(
        "SELECT id, sku, name, price_amount, price_currency, active
         FROM inventory.products ORDER BY sku LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(id, sku, name, price_amount, price_currency, active)| ProductRow {
                id,
                sku,
                name,
                price_amount,
                price_currency,
                active,
            },
        )
        .collect())
}

pub async fn count_products(pool: &PgPool, tenant_id: TenantId) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM inventory.products")
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(count)
}

pub async fn find_products_by_ids(
    pool: &PgPool,
    tenant_id: TenantId,
    ids: &[Uuid],
) -> Result<Vec<ProductRow>, PostgresError> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (Uuid, String, String, i64, String, bool)>(
        "SELECT id, sku, name, price_amount, price_currency, active
         FROM inventory.products WHERE id = ANY($1)",
    )
    .bind(ids)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(id, sku, name, price_amount, price_currency, active)| ProductRow {
                id,
                sku,
                name,
                price_amount,
                price_currency,
                active,
            },
        )
        .collect())
}

pub struct StockMovementInsert {
    pub id: Uuid,
    pub product_id: Uuid,
    pub responsible_id: Uuid,
    pub movement_type: String,
    pub quantity: i32,
    pub reference_id: Option<Uuid>,
}

pub async fn insert_stock_movement(
    pool: &PgPool,
    tenant_id: TenantId,
    movement: StockMovementInsert,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO inventory.stock_movements
         (id, tenant_id, product_id, responsible_id, movement_type, quantity, reference_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(movement.id)
    .bind(tenant_id.as_uuid())
    .bind(movement.product_id)
    .bind(movement.responsible_id)
    .bind(movement.movement_type)
    .bind(movement.quantity)
    .bind(movement.reference_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn decrement_stock_balance(
    pool: &PgPool,
    tenant_id: TenantId,
    driver_id: Uuid,
    product_id: Uuid,
    quantity: i32,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE inventory.stock_balances
         SET quantity = quantity - $4, updated_at = now()
         WHERE tenant_id = $1 AND driver_id = $2 AND product_id = $3 AND quantity >= $4",
    )
    .bind(tenant_id.as_uuid())
    .bind(driver_id)
    .bind(product_id)
    .bind(quantity)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
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
