//! Dev tenant site branding — Phase 41.

use domain_shared::TenantId;
use infra_postgres::PgPool;
use uuid::Uuid;

use crate::error::DevSeedResult;
use crate::ids::admin_user_id;
use crate::seed_assets;

const DEV_SITE_LOGO_FILE_ID: &str = "01900001-0010-7000-8000-000000000098";
const SITE_LOGO_OBJECT_KEY: &str = "tenants/site-logo.png";
const SITE_LOGO_ASSET: &str = "tenants/site-logo.png";
const DEV_SITE_DISPLAY_NAME: &str = "Burger Delicious";

pub async fn seed_tenant_site_settings(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
) -> DevSeedResult<()> {
    ensure_tenant_site_settings(app_pool, admin_pool, tenant).await
}

pub async fn ensure_tenant_site_settings(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
) -> DevSeedResult<()> {
    let logo_file_id = Uuid::parse_str(DEV_SITE_LOGO_FILE_ID).expect("site logo id");
    seed_assets::ensure_media_file(
        app_pool,
        admin_pool,
        tenant,
        logo_file_id,
        "Tenant",
        tenant.as_uuid(),
        SITE_LOGO_OBJECT_KEY,
        SITE_LOGO_ASSET,
        admin_user_id(),
    )
    .await?;

    infra_postgres::shared::update_tenant_display_name(app_pool, tenant, DEV_SITE_DISPLAY_NAME)
        .await?;
    infra_postgres::shared::update_tenant_logo(app_pool, tenant, Some(logo_file_id)).await?;
    infra_postgres::shared::update_tenant_sales_contact_phone(
        app_pool,
        tenant,
        Some("5511987654321"),
    )
    .await?;

    Ok(())
}
