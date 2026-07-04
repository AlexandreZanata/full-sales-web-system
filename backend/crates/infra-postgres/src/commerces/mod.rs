use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

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
