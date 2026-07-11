//! Portal home content seed — hero banners, featured flags, popular metrics.

use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::inventory::portal_products::{seed_product_sales_total, set_product_featured};
use infra_postgres::portal::banners::{self, BannerInsert};

use crate::catalog::CatalogSeed;
use crate::error::DevSeedResult;
use crate::ids::{admin_user_id, portal_banner_file_ids, portal_banner_ids};
use crate::seed_assets::{ensure_media_file, ensure_storage_bytes};

const BANNER_SPECS: [(&str, &str, &str, i32); 3] = [
    (
        "banners/hero-burger.png",
        "portal/banners/hero-burger.png",
        "Combo burger — peça agora",
        0,
    ),
    (
        "banners/hero-fresh-food.png",
        "portal/banners/hero-fresh-food.png",
        "Ingredientes frescos todos os dias",
        1,
    ),
    (
        "banners/hero-breakfast.png",
        "portal/banners/hero-breakfast.png",
        "Café da manhã especial",
        2,
    ),
];

pub async fn seed_portal_home_content(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
    catalog: &CatalogSeed,
) -> DevSeedResult<()> {
    seed_featured_products(app_pool, tenant, catalog).await?;
    seed_popular_metrics(app_pool, tenant, catalog).await?;
    seed_hero_banners(app_pool, admin_pool, tenant).await?;
    Ok(())
}

pub async fn ensure_portal_home_content(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
    catalog: &CatalogSeed,
) -> DevSeedResult<()> {
    seed_portal_home_content(app_pool, admin_pool, tenant, catalog).await
}

async fn seed_featured_products(
    app_pool: &PgPool,
    tenant: TenantId,
    catalog: &CatalogSeed,
) -> DevSeedResult<()> {
    for product_id in catalog.product_ids.iter().take(6) {
        set_product_featured(app_pool, tenant, *product_id, true).await?;
    }
    Ok(())
}

async fn seed_popular_metrics(
    app_pool: &PgPool,
    tenant: TenantId,
    catalog: &CatalogSeed,
) -> DevSeedResult<()> {
    let weights = [180_i64, 150, 120, 95, 80, 70, 55, 40, 30, 25, 20, 15];
    for (product_id, units) in catalog.product_ids.iter().zip(weights) {
        seed_product_sales_total(app_pool, tenant, *product_id, units).await?;
    }
    Ok(())
}

async fn seed_hero_banners(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
) -> DevSeedResult<()> {
    let banner_ids = portal_banner_ids();
    let file_ids = portal_banner_file_ids();
    let uploader = admin_user_id();

    for (index, (asset, object_key, alt_text, sort_order)) in BANNER_SPECS.iter().enumerate() {
        let banner_id = banner_ids[index];
        let file_id = file_ids[index];
        ensure_media_file(
            app_pool,
            admin_pool,
            tenant,
            file_id,
            "PortalBanner",
            banner_id,
            object_key,
            asset,
            uploader,
        )
        .await?;
        if banners::find_banner_by_id(app_pool, tenant, banner_id)
            .await?
            .is_some()
        {
            banners::update_banner(
                app_pool,
                tenant,
                banner_id,
                &banners::BannerUpdate {
                    placement: None,
                    image_file_id: Some(Some(file_id)),
                    image_url: Some(None),
                    link_url: None,
                    alt_text: Some(Some((*alt_text).into())),
                    sort_order: Some(*sort_order),
                    active: Some(true),
                },
            )
            .await?;
            continue;
        }
        banners::insert_banner(
            app_pool,
            tenant,
            BannerInsert {
                id: banner_id,
                placement: "hero".into(),
                image_file_id: Some(file_id),
                image_url: None,
                link_url: None,
                alt_text: Some((*alt_text).into()),
                sort_order: *sort_order,
                active: true,
            },
        )
        .await?;
    }
    ensure_portal_banner_storage().await?;
    Ok(())
}

async fn ensure_portal_banner_storage() -> DevSeedResult<()> {
    use crate::seed_assets::read_asset_or_placeholder;
    for (asset, object_key, _, _) in BANNER_SPECS {
        let (bytes, mime) = read_asset_or_placeholder(asset);
        ensure_storage_bytes(object_key, &bytes, mime).await?;
    }
    Ok(())
}
