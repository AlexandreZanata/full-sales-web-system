use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct ProductStockOverviewRow {
    pub product_id: Uuid,
    pub sku: String,
    pub name: String,
    pub unit_of_measure: String,
    pub active: bool,
    pub balance_total: i32,
    pub reserved: i32,
    pub available: i32,
}

pub async fn list_product_stock_overview(
    pool: &PgPool,
    tenant_id: TenantId,
    search: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProductStockOverviewRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let pattern = search.map(|term| format!("%{}%", term.trim()));
    let rows = sqlx::query_as::<_, (Uuid, String, String, String, bool, i64, i64)>(
        "SELECT p.id, p.sku, p.name, p.unit_of_measure, p.active,
                COALESCE((
                    SELECT SUM(sb.quantity)::bigint
                    FROM inventory.stock_balances sb
                    WHERE sb.product_id = p.id
                ), 0),
                COALESCE((
                    SELECT SUM(sr.quantity_reserved)::bigint
                    FROM inventory.stock_reservations sr
                    WHERE sr.product_id = p.id AND sr.status = 'Active'
                ), 0)
         FROM inventory.products p
         WHERE ($1::text IS NULL OR p.sku ILIKE $1 OR p.name ILIKE $1)
         ORDER BY p.sku
         LIMIT $2 OFFSET $3",
    )
    .bind(pattern)
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(map_stock_overview_rows(rows))
}

pub async fn list_product_stock_overview_cursor(
    pool: &PgPool,
    tenant_id: TenantId,
    name_like: Option<&str>,
    sku_like: Option<&str>,
    after_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<ProductStockOverviewRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (Uuid, String, String, String, bool, i64, i64)>(
        "SELECT p.id, p.sku, p.name, p.unit_of_measure, p.active,
                COALESCE((
                    SELECT SUM(sb.quantity)::bigint
                    FROM inventory.stock_balances sb
                    WHERE sb.product_id = p.id
                ), 0),
                COALESCE((
                    SELECT SUM(sr.quantity_reserved)::bigint
                    FROM inventory.stock_reservations sr
                    WHERE sr.product_id = p.id AND sr.status = 'Active'
                ), 0)
         FROM inventory.products p
         WHERE ($1::text IS NULL OR p.name ILIKE $1)
           AND ($2::text IS NULL OR p.sku ILIKE $2)
           AND ($3::uuid IS NULL OR p.id > $3)
         ORDER BY p.id ASC
         LIMIT $4",
    )
    .bind(name_like)
    .bind(sku_like)
    .bind(after_id)
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(map_stock_overview_rows(rows))
}

fn map_stock_overview_rows(
    rows: Vec<(Uuid, String, String, String, bool, i64, i64)>,
) -> Vec<ProductStockOverviewRow> {
    rows.into_iter()
        .map(
            |(product_id, sku, name, unit_of_measure, active, balance_total, reserved)| {
                let balance_total = balance_total as i32;
                let reserved = reserved as i32;
                ProductStockOverviewRow {
                    product_id,
                    sku,
                    name,
                    unit_of_measure,
                    active,
                    balance_total,
                    reserved,
                    available: (balance_total - reserved).max(0),
                }
            },
        )
        .collect()
}

pub async fn count_product_stock_overview(
    pool: &PgPool,
    tenant_id: TenantId,
    search: Option<&str>,
) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let pattern = search.map(|term| format!("%{}%", term.trim()));
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)
         FROM inventory.products p
         WHERE ($1::text IS NULL OR p.sku ILIKE $1 OR p.name ILIKE $1)",
    )
    .bind(pattern)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(total)
}
