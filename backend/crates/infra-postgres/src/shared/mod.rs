pub mod offboarding;
pub mod tenant_stats;
pub mod tenants_lifecycle;

pub use tenant_stats::{
    TenantWorkforceStats, admin_session_for_tenant, tenant_workforce_stats,
    update_tenant_feature_flags,
};

pub use offboarding::{anonymize_tenant_pii, find_offboarding_candidates};
pub use tenants_lifecycle::{
    ProvisionTenantParams, TenantCounts, TenantLifecycleRow, TenantListRow, backdate_past_due_at,
    find_dunning_candidates, find_tenant_lifecycle, find_tenant_status, list_tenants_platform,
    mark_tenant_past_due, mark_tenant_payment_cleared, plan_exists, provision_tenant,
    set_grace_extended_until, tenant_counts, update_tenant_lifecycle,
};

use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;

/// Row persisted in `shared.tenants`.
#[derive(Debug, Clone)]
pub struct TenantRow {
    pub id: TenantId,
    pub name: String,
    pub display_name: String,
    pub logo_file_id: Option<Uuid>,
    pub sales_contact_phone: Option<String>,
    pub active: bool,
}

/// Inserts a tenant (admin/migration path — no RLS on `shared.tenants`).
pub async fn insert_tenant(pool: &PgPool, id: TenantId, name: &str) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO shared.tenants (id, name, legal_name, display_name, status, active)
         VALUES ($1, $2, $2, $2, 'Active', true)",
    )
    .bind(id.as_uuid())
    .bind(name)
    .execute(pool)
    .await?;
    Ok(())
}

/// Finds a tenant by id.
pub async fn find_tenant_by_id(
    pool: &PgPool,
    id: TenantId,
) -> Result<Option<TenantRow>, PostgresError> {
    let row = sqlx::query_as::<_, TenantRecord>(
        "SELECT id, name, display_name, logo_file_id, sales_contact_phone, active FROM shared.tenants WHERE id = $1",
    )
    .bind(id.as_uuid())
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| TenantRow {
        id: TenantId::from_uuid(r.id),
        name: r.name,
        display_name: r.display_name,
        logo_file_id: r.logo_file_id,
        sales_contact_phone: r.sales_contact_phone,
        active: r.active,
    }))
}

pub async fn update_tenant_display_name(
    pool: &PgPool,
    tenant_id: TenantId,
    display_name: &str,
) -> Result<bool, PostgresError> {
    let result = sqlx::query("UPDATE shared.tenants SET display_name = $1 WHERE id = $2")
        .bind(display_name)
        .bind(tenant_id.as_uuid())
        .execute(pool)
        .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn update_tenant_logo(
    pool: &PgPool,
    tenant_id: TenantId,
    logo_file_id: Option<Uuid>,
) -> Result<bool, PostgresError> {
    let result = sqlx::query("UPDATE shared.tenants SET logo_file_id = $1 WHERE id = $2")
        .bind(logo_file_id)
        .bind(tenant_id.as_uuid())
        .execute(pool)
        .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn find_tenant_site_logo_media(
    pool: &PgPool,
    tenant_id: TenantId,
    file_id: Uuid,
) -> Result<Option<crate::inventory::product_images::PublicProductMediaRow>, PostgresError> {
    use crate::inventory::product_images::PublicProductMediaRow;
    use crate::rls::apply_tenant_context;

    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (String, String, String)>(
        "SELECT mf.bucket, mf.object_key, mf.mime_type
         FROM media.files mf
         JOIN shared.tenants t ON t.logo_file_id = mf.id
         WHERE mf.id = $1 AND mf.entity_type = 'Tenant' AND t.id = $2 AND t.active = true",
    )
    .bind(file_id)
    .bind(tenant_id.as_uuid())
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(
        row.map(|(bucket, object_key, mime_type)| PublicProductMediaRow {
            bucket,
            object_key,
            mime_type,
        }),
    )
}

pub async fn update_tenant_sales_contact_phone(
    pool: &PgPool,
    tenant_id: TenantId,
    sales_contact_phone: Option<&str>,
) -> Result<bool, PostgresError> {
    let result = sqlx::query("UPDATE shared.tenants SET sales_contact_phone = $1 WHERE id = $2")
        .bind(sales_contact_phone)
        .bind(tenant_id.as_uuid())
        .execute(pool)
        .await?;
    Ok(result.rows_affected() == 1)
}

#[derive(sqlx::FromRow)]
struct TenantRecord {
    id: Uuid,
    name: String,
    display_name: String,
    logo_file_id: Option<Uuid>,
    sales_contact_phone: Option<String>,
    active: bool,
}
