use domain_shared::TenantId;
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

#[derive(FromRow)]
pub struct TopSellingProductRow {
    pub product_id: Uuid,
    pub name: String,
    pub sku: String,
    pub units_sold: i64,
}

pub async fn record_product_sales_in_tx(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: Uuid,
    items: &[(Uuid, i32)],
) -> Result<(), sqlx::Error> {
    for (product_id, quantity) in items {
        sqlx::query(
            "INSERT INTO sales.product_sales_totals (tenant_id, product_id, units_sold, updated_at)
             VALUES ($1, $2, $3, now())
             ON CONFLICT (tenant_id, product_id)
             DO UPDATE SET
               units_sold = sales.product_sales_totals.units_sold + EXCLUDED.units_sold,
               updated_at = now()",
        )
        .bind(tenant_id)
        .bind(product_id)
        .bind(i64::from(*quantity))
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

pub async fn list_top_selling_products(
    pool: &PgPool,
    tenant_id: TenantId,
    limit: i64,
) -> Result<Vec<TopSellingProductRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, TopSellingProductRow>(
        "SELECT pst.product_id, p.name, p.sku, pst.units_sold
         FROM sales.product_sales_totals pst
         INNER JOIN inventory.products p
           ON p.id = pst.product_id AND p.tenant_id = pst.tenant_id
         WHERE pst.tenant_id = $1 AND p.active = true
         ORDER BY pst.units_sold DESC, p.name ASC
         LIMIT $2",
    )
    .bind(tenant_id.as_uuid())
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows)
}
