use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use super::{ProductDbRow, ProductRow, map_product_row, PRODUCT_SELECT};
use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub async fn list_portal_featured_products(
    pool: &PgPool,
    tenant_id: TenantId,
    limit: i64,
) -> Result<Vec<ProductRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, ProductDbRow>(&format!(
        "{PRODUCT_SELECT}
         WHERE p.active = true AND p.is_featured = true
         ORDER BY p.sku ASC
         LIMIT $1"
    ))
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows.into_iter().map(map_product_row).collect())
}

pub async fn list_portal_popular_products(
    pool: &PgPool,
    tenant_id: TenantId,
    limit: i64,
) -> Result<Vec<ProductRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, ProductDbRow>(&format!(
        "{PRODUCT_SELECT}
         LEFT JOIN sales.product_sales_totals pst
           ON pst.product_id = p.id AND pst.tenant_id = p.tenant_id
         WHERE p.active = true
         ORDER BY COALESCE(pst.units_sold, 0) DESC, p.sku ASC
         LIMIT $1"
    ))
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows.into_iter().map(map_product_row).collect())
}

pub async fn set_product_featured(
    pool: &PgPool,
    tenant_id: TenantId,
    product_id: Uuid,
    is_featured: bool,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE inventory.products SET is_featured = $3, updated_at = now()
         WHERE tenant_id = $1 AND id = $2",
    )
    .bind(tenant_id.as_uuid())
    .bind(product_id)
    .bind(is_featured)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn seed_product_sales_total(
    pool: &PgPool,
    tenant_id: TenantId,
    product_id: Uuid,
    units_sold: i64,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO sales.product_sales_totals (tenant_id, product_id, units_sold, updated_at)
         VALUES ($1, $2, $3, now())
         ON CONFLICT (tenant_id, product_id)
         DO UPDATE SET units_sold = EXCLUDED.units_sold, updated_at = now()",
    )
    .bind(tenant_id.as_uuid())
    .bind(product_id)
    .bind(units_sold)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}
