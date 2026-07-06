use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

use super::{CommerceInsert, CommerceRow};

#[allow(clippy::type_complexity)]
pub(crate) type CommerceRowRecord = (
    uuid::Uuid,
    String,
    String,
    String,
    bool,
    Option<uuid::Uuid>,
    String,
    Option<uuid::Uuid>,
    Option<uuid::Uuid>,
    Option<String>,
    Option<serde_json::Value>,
    Option<String>,
);

pub struct RegistrationInsert<'a> {
    pub commerce: CommerceInsert<'a>,
    pub delivery_address: super::addresses::AddressInsert,
}

pub async fn commerce_cnpj_exists(
    pool: &PgPool,
    tenant_id: TenantId,
    cnpj: &str,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM commerces.commerces WHERE cnpj = $1)",
    )
    .bind(cnpj)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(exists)
}

pub async fn insert_registration(
    pool: &PgPool,
    tenant_id: TenantId,
    row: RegistrationInsert<'_>,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    insert_commerce_row(&mut tx, tenant_id, &row.commerce).await?;
    super::addresses::insert_address_in_tx(&mut tx, tenant_id, &row.delivery_address).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn find_commerce_registration_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<CommerceRow>, PostgresError> {
    find_commerce_row(pool, tenant_id, id, None, None).await
}

pub async fn list_registrations_cursor(
    pool: &PgPool,
    tenant_id: TenantId,
    status: Option<&str>,
    submitted_by: Option<Uuid>,
    after_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<CommerceRow>, PostgresError> {
    find_commerce_rows(pool, tenant_id, None, status, submitted_by, after_id, limit).await
}

pub async fn update_registration_fields(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    legal_name: &str,
    trade_name: &str,
    address: serde_json::Value,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE commerces.commerces
         SET legal_name = $2, trade_name = $3, address = $4, updated_at = now()
         WHERE id = $1 AND registration_status = 'PendingReview'",
    )
    .bind(id)
    .bind(legal_name)
    .bind(trade_name)
    .bind(address)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn approve_registration(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    reviewed_by: Uuid,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE commerces.commerces
         SET active = true,
             registration_status = 'Active',
             reviewed_by_user_id = $2,
             rejection_reason = NULL,
             updated_at = now()
         WHERE id = $1 AND registration_status = 'PendingReview'",
    )
    .bind(id)
    .bind(reviewed_by)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn reject_registration(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    reviewed_by: Uuid,
    reason: &str,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query(
        "UPDATE commerces.commerces
         SET active = false,
             registration_status = 'Rejected',
             reviewed_by_user_id = $2,
             rejection_reason = $3,
             updated_at = now()
         WHERE id = $1 AND registration_status = 'PendingReview'",
    )
    .bind(id)
    .bind(reviewed_by)
    .bind(reason)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

async fn find_commerce_row(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    status: Option<&str>,
    submitted_by: Option<Uuid>,
) -> Result<Option<CommerceRow>, PostgresError> {
    let rows = find_commerce_rows(pool, tenant_id, Some(id), status, submitted_by, None, 1).await?;
    Ok(rows.into_iter().next())
}

async fn find_commerce_rows(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Option<Uuid>,
    status: Option<&str>,
    submitted_by: Option<Uuid>,
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
         WHERE ($1::uuid IS NULL OR id = $1)
           AND ($2::text IS NULL OR registration_status = $2)
           AND ($3::uuid IS NULL OR submitted_by_user_id = $3)
           AND ($4::uuid IS NULL OR id > $4)
         ORDER BY id ASC
         LIMIT $5",
    )
    .bind(id)
    .bind(status)
    .bind(submitted_by)
    .bind(after_id)
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows.into_iter().map(map_commerce_row).collect())
}

pub(crate) fn map_commerce_row(
    (
        id,
        cnpj,
        legal_name,
        trade_name,
        active,
        logo_file_id,
        registration_status,
        submitted_by_user_id,
        reviewed_by_user_id,
        rejection_reason,
        lookup_snapshot,
        registration_mode,
    ): CommerceRowRecord,
) -> CommerceRow {
    CommerceRow {
        id,
        cnpj,
        legal_name,
        trade_name,
        active,
        logo_file_id,
        registration_status,
        submitted_by_user_id,
        reviewed_by_user_id,
        rejection_reason,
        lookup_snapshot,
        registration_mode,
    }
}

pub async fn insert_commerce_row(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    tenant_id: TenantId,
    row: &CommerceInsert<'_>,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO commerces.commerces
         (id, tenant_id, cnpj, legal_name, trade_name, address, active,
          registration_status, submitted_by_user_id, reviewed_by_user_id,
          rejection_reason, lookup_snapshot, registration_mode)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
    )
    .bind(row.id)
    .bind(tenant_id.as_uuid())
    .bind(row.cnpj)
    .bind(row.legal_name)
    .bind(row.trade_name)
    .bind(&row.address)
    .bind(row.active)
    .bind(row.registration_status)
    .bind(row.submitted_by_user_id)
    .bind(row.reviewed_by_user_id)
    .bind(row.rejection_reason)
    .bind(&row.lookup_snapshot)
    .bind(row.registration_mode)
    .execute(&mut **tx)
    .await?;
    Ok(())
}
