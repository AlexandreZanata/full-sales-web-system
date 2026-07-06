use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

use super::{CommerceRow, map_commerce_tuple};

pub(crate) use super::registrations::CommerceRowRecord;

pub async fn find_commerce_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<CommerceRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, CommerceRowRecord>(
        "SELECT id, cnpj, legal_name, trade_name, active, logo_file_id,
                registration_status, submitted_by_user_id, reviewed_by_user_id,
                rejection_reason, lookup_snapshot, registration_mode
         FROM commerces.commerces WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(map_commerce_tuple))
}

pub async fn list_commerces_cursor(
    pool: &PgPool,
    tenant_id: TenantId,
    active: Option<bool>,
    after_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<CommerceRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, CommerceRowRecord>(
        "SELECT id, cnpj, legal_name, trade_name, active, logo_file_id,
                registration_status, submitted_by_user_id, reviewed_by_user_id,
                rejection_reason, lookup_snapshot, registration_mode
         FROM commerces.commerces
         WHERE ($1::bool IS NULL OR active = $1)
           AND ($2::uuid IS NULL OR id > $2)
         ORDER BY id ASC
         LIMIT $3",
    )
    .bind(active)
    .bind(after_id)
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows.into_iter().map(map_commerce_tuple).collect())
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

pub async fn list_commerces(
    pool: &PgPool,
    tenant_id: TenantId,
    active: Option<bool>,
    limit: i64,
    offset: i64,
) -> Result<Vec<CommerceRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, CommerceRowRecord>(
        "SELECT id, cnpj, legal_name, trade_name, active, logo_file_id,
                registration_status, submitted_by_user_id, reviewed_by_user_id,
                rejection_reason, lookup_snapshot, registration_mode
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
    Ok(rows.into_iter().map(map_commerce_tuple).collect())
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
