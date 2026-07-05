//! Idempotent dev database seed for the admin panel.

pub mod media_bytes;

mod audit;
mod catalog;
mod commerces;
mod deliveries;
mod error;
mod foundation;
pub mod ids;
mod orders;
mod reports;
mod sales;
mod site_settings;
mod users;

pub use error::{DevSeedError, DevSeedResult};
pub use foundation::{is_already_seeded, signing_key_from_env};
pub use media_bytes::minimal_webp_bytes;

use infra_postgres::PgPool;

/// Connection pair mirroring api-http (`DATABASE_ADMIN_URL` + `DATABASE_URL`).
pub struct SeedPools {
    pub admin: PgPool,
    pub app: PgPool,
}

/// Populates the dev tenant with realistic data for every admin screen.
pub async fn seed_dev_dataset(pools: &SeedPools) -> DevSeedResult<()> {
    if is_already_seeded(&pools.admin, &pools.app).await? {
        tracing::info!("dev seed already applied — skipping");
        catalog::ensure_catalog_storage_objects().await?;
        return Ok(());
    }

    let foundation = foundation::seed_foundation(&pools.admin, &pools.app)
        .await
        .map_err(|e| wrap_step("foundation", e))?;
    site_settings::seed_tenant_site_settings(&pools.app, foundation.tenant_id)
        .await
        .map_err(|e| wrap_step("site_settings", e))?;
    let users = users::seed_users(&pools.admin, &pools.app, foundation.tenant_id)
        .await
        .map_err(|e| wrap_step("users", e))?;
    let commerces = commerces::seed_commerces(&pools.app, foundation.tenant_id)
        .await
        .map_err(|e| wrap_step("commerces", e))?;
    let catalog = catalog::seed_catalog(&pools.app, foundation.tenant_id, &users, &commerces)
        .await
        .map_err(|e| wrap_step("catalog", e))?;
    orders::seed_orders(
        &pools.app,
        foundation.tenant_id,
        &users,
        &commerces,
        &catalog,
    )
    .await
    .map_err(|e| wrap_step("orders", e))?;
    deliveries::seed_deliveries(&pools.app, foundation.tenant_id, &users)
        .await
        .map_err(|e| wrap_step("deliveries", e))?;
    sales::seed_sales(
        &pools.app,
        foundation.tenant_id,
        &users,
        &commerces,
        &catalog,
    )
    .await
    .map_err(|e| wrap_step("sales", e))?;
    reports::seed_reports(&pools.app, &foundation, &users, &commerces)
        .await
        .map_err(|e| wrap_step("reports", e))?;
    audit::seed_audit(
        &pools.app,
        foundation.tenant_id,
        &users,
        &commerces,
        &catalog,
    )
    .await
    .map_err(|e| wrap_step("audit", e))?;

    tracing::info!("dev seed completed for tenant {}", foundation.tenant_id);
    Ok(())
}

/// Returns true when `ALLOW_DEV_SEED=1`.
pub fn dev_seed_allowed() -> bool {
    std::env::var("ALLOW_DEV_SEED")
        .map(|value| value == "1")
        .unwrap_or(false)
}

fn wrap_step(step: &str, err: DevSeedError) -> DevSeedError {
    DevSeedError::Aborted(format!("{step}: {err}"))
}
