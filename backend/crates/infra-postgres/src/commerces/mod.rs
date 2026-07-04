use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub mod addresses;

pub async fn insert_commerce(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    cnpj: &str,
    legal_name: &str,
    trade_name: &str,
    address: serde_json::Value,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO commerces.commerces (id, tenant_id, cnpj, legal_name, trade_name, address)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(id)
    .bind(tenant_id.as_uuid())
    .bind(cnpj)
    .bind(legal_name)
    .bind(trade_name)
    .bind(address)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn list_commerce_ids(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<Uuid>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_scalar::<_, Uuid>("SELECT id FROM commerces.commerces ORDER BY cnpj")
        .fetch_all(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(rows)
}

pub struct CommerceRow {
    pub id: Uuid,
    pub cnpj: String,
    pub legal_name: String,
    pub trade_name: String,
    pub active: bool,
}

pub async fn find_commerce_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<CommerceRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (Uuid, String, String, String, bool)>(
        "SELECT id, cnpj, legal_name, trade_name, active
         FROM commerces.commerces WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(
        row.map(|(id, cnpj, legal_name, trade_name, active)| CommerceRow {
            id,
            cnpj,
            legal_name,
            trade_name,
            active,
        }),
    )
}

pub async fn list_commerces(
    pool: &PgPool,
    tenant_id: TenantId,
    active: Option<bool>,
    limit: i64,
    offset: i64,
) -> Result<Vec<CommerceRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (Uuid, String, String, String, bool)>(
        "SELECT id, cnpj, legal_name, trade_name, active
         FROM commerces.commerces
         WHERE ($1::bool IS NULL OR active = $1)
         ORDER BY cnpj LIMIT $2 OFFSET $3",
    )
    .bind(active)
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(|(id, cnpj, legal_name, trade_name, active)| CommerceRow {
            id,
            cnpj,
            legal_name,
            trade_name,
            active,
        })
        .collect())
}

pub async fn count_commerces(
    pool: &PgPool,
    tenant_id: TenantId,
    active: Option<bool>,
) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM commerces.commerces WHERE ($1::bool IS NULL OR active = $1)",
    )
    .bind(active)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(count)
}

/// Deactivates a commerce under tenant RLS (sets `active = false`).
pub async fn deactivate_commerce(
    pool: &PgPool,
    tenant_id: TenantId,
    commerce_id: Uuid,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE commerces.commerces SET active = false WHERE id = $1 AND active = true",
    )
    .bind(commerce_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}
