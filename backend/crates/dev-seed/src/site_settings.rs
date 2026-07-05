//! Dev tenant site branding — Phase 41.

use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::media::{self, FileInsert};
use uuid::Uuid;

use crate::error::DevSeedResult;
use crate::ids::admin_user_id;
use crate::media_bytes::{DEV_MEDIA_BUCKET, minimal_webp_bytes};

const DEV_SITE_LOGO_FILE_ID: &str = "01900001-0010-7000-8000-000000000098";

pub async fn seed_tenant_site_settings(app_pool: &PgPool, tenant: TenantId) -> DevSeedResult<()> {
    let logo_file_id = Uuid::parse_str(DEV_SITE_LOGO_FILE_ID).expect("site logo id");
    seed_site_logo_file(app_pool, tenant, logo_file_id).await?;

    infra_postgres::shared::update_tenant_display_name(app_pool, tenant, "Dev Sales Platform")
        .await?;
    infra_postgres::shared::update_tenant_logo(app_pool, tenant, Some(logo_file_id)).await?;
    infra_postgres::shared::update_tenant_sales_contact_phone(app_pool, tenant, Some("5511987654321"))
        .await?;

    Ok(())
}

async fn seed_site_logo_file(
    app_pool: &PgPool,
    tenant: TenantId,
    file_id: Uuid,
) -> DevSeedResult<()> {
    if media::find_file_by_id(app_pool, tenant, file_id)
        .await?
        .is_some()
    {
        return Ok(());
    }

    let bytes = minimal_webp_bytes();
    media::insert_file(
        app_pool,
        tenant,
        FileInsert {
            id: file_id,
            entity_type: "Tenant".into(),
            entity_id: tenant.as_uuid(),
            bucket: DEV_MEDIA_BUCKET.into(),
            object_key: "tenants/dev-site-logo.webp".into(),
            mime_type: "image/webp".into(),
            size_bytes: bytes.len() as i64,
            sha256: "dev-seed-site-logo-sha256".into(),
            uploaded_by_user_id: admin_user_id(),
        },
    )
    .await?;

    Ok(())
}
