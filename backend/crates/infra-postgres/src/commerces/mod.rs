use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub mod addresses;
pub mod read;
pub mod registrations;

pub use read::{
    count_commerces, find_commerce_by_id, list_commerce_ids, list_commerces, list_commerces_cursor,
};

pub struct CommerceRow {
    pub id: Uuid,
    pub cnpj: String,
    pub legal_name: String,
    pub trade_name: String,
    pub active: bool,
    pub logo_file_id: Option<Uuid>,
    pub registration_status: String,
    pub submitted_by_user_id: Option<Uuid>,
    pub reviewed_by_user_id: Option<Uuid>,
    pub rejection_reason: Option<String>,
    pub lookup_snapshot: Option<serde_json::Value>,
    pub registration_mode: Option<String>,
}

pub struct CommerceInsert<'a> {
    pub id: Uuid,
    pub cnpj: &'a str,
    pub legal_name: &'a str,
    pub trade_name: &'a str,
    pub address: serde_json::Value,
    pub active: bool,
    pub registration_status: &'a str,
    pub submitted_by_user_id: Option<Uuid>,
    pub reviewed_by_user_id: Option<Uuid>,
    pub rejection_reason: Option<&'a str>,
    pub lookup_snapshot: Option<serde_json::Value>,
    pub registration_mode: Option<&'a str>,
}

pub(crate) fn map_commerce_tuple(row: registrations::CommerceRowRecord) -> CommerceRow {
    registrations::map_commerce_row(row)
}

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
    registrations::insert_commerce_row(
        &mut tx,
        tenant_id,
        &CommerceInsert {
            id,
            cnpj,
            legal_name,
            trade_name,
            address,
            active: true,
            registration_status: "Active",
            submitted_by_user_id: None,
            reviewed_by_user_id: None,
            rejection_reason: None,
            lookup_snapshot: None,
            registration_mode: None,
        },
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

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
