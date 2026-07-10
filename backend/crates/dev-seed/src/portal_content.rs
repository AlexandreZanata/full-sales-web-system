//! Portal home content seed — hero banners, promotions, featured flags, popular metrics.

use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::inventory::portal_products::{seed_product_sales_total, set_product_featured};
use infra_postgres::media::{self, FileInsert};
use infra_postgres::portal::banners::{self, BannerInsert};
use infra_postgres::portal::promotions::{self, PromotionInsert};
use infra_storage::{LocalFsObjectStorage, ObjectStorage};
use uuid::Uuid;

use crate::catalog::CatalogSeed;
use crate::error::DevSeedResult;
use crate::ids::{admin_user_id, portal_banner_file_ids, portal_banner_ids, portal_promotion_ids};
use crate::media_bytes::{DEV_MEDIA_BUCKET, minimal_webp_bytes};

pub async fn seed_portal_home_content(
    app_pool: &PgPool,
    tenant: TenantId,
    catalog: &CatalogSeed,
) -> DevSeedResult<()> {
    seed_featured_products(app_pool, tenant, catalog).await?;
    seed_popular_metrics(app_pool, tenant, catalog).await?;
    seed_hero_banners(app_pool, tenant).await?;
    seed_promotions(app_pool, tenant).await?;
    Ok(())
}

pub async fn ensure_portal_home_content(
    app_pool: &PgPool,
    tenant: TenantId,
    catalog: &CatalogSeed,
) -> DevSeedResult<()> {
    seed_portal_home_content(app_pool, tenant, catalog).await
}

async fn seed_featured_products(
    app_pool: &PgPool,
    tenant: TenantId,
    catalog: &CatalogSeed,
) -> DevSeedResult<()> {
    for product_id in catalog.product_ids.iter().take(3) {
        set_product_featured(app_pool, tenant, *product_id, true).await?;
    }
    Ok(())
}

async fn seed_popular_metrics(
    app_pool: &PgPool,
    tenant: TenantId,
    catalog: &CatalogSeed,
) -> DevSeedResult<()> {
    let weights = [120_i64, 85, 60, 25];
    for (product_id, units) in catalog.product_ids.iter().zip(weights) {
        seed_product_sales_total(app_pool, tenant, *product_id, units).await?;
    }
    Ok(())
}

async fn seed_hero_banners(app_pool: &PgPool, tenant: TenantId) -> DevSeedResult<()> {
    let banner_ids = portal_banner_ids();
    let file_ids = portal_banner_file_ids();
    let specs = [
        ("portal/banners/hero-1.webp", "Welcome to our menu", 0_i32),
        ("portal/banners/hero-2.webp", "Fresh deals every day", 1_i32),
    ];

    for (index, (object_key, alt_text, sort_order)) in specs.iter().enumerate() {
        let banner_id = banner_ids[index];
        let file_id = file_ids[index];
        insert_banner_file(app_pool, tenant, file_id, banner_id, object_key).await?;
        if banners::find_banner_by_id(app_pool, tenant, banner_id)
            .await?
            .is_some()
        {
            continue;
        }
        banners::insert_banner(
            app_pool,
            tenant,
            BannerInsert {
                id: banner_id,
                placement: "hero".into(),
                image_file_id: file_id,
                link_url: None,
                alt_text: Some((*alt_text).into()),
                sort_order: *sort_order,
                active: true,
            },
        )
        .await?;
    }
    Ok(())
}

async fn seed_promotions(app_pool: &PgPool, tenant: TenantId) -> DevSeedResult<()> {
    let promotion_ids = portal_promotion_ids();
    let specs = [
        (
            promotion_ids[0],
            "Tasty Burger",
            "30% OFF",
            "yellow",
            Some("snacks"),
            0_i32,
        ),
        (
            promotion_ids[1],
            "Fresh Salad",
            "15% OFF",
            "green",
            Some("bebidas"),
            1_i32,
        ),
    ];

    for (id, headline, discount_text, background, category_slug, sort_order) in specs {
        if promotions::find_promotion_by_id(app_pool, tenant, id)
            .await?
            .is_some()
        {
            continue;
        }
        promotions::insert_promotion(
            app_pool,
            tenant,
            PromotionInsert {
                id,
                headline: headline.into(),
                discount_text: discount_text.into(),
                background: background.into(),
                category_slug: category_slug.map(str::to_string),
                link_url: None,
                image_file_id: None,
                sort_order,
                active: true,
            },
        )
        .await?;
    }
    Ok(())
}

async fn insert_banner_file(
    app_pool: &PgPool,
    tenant: TenantId,
    file_id: Uuid,
    banner_id: Uuid,
    object_key: &str,
) -> DevSeedResult<()> {
    if media::find_file_by_id(app_pool, tenant, file_id)
        .await?
        .is_some()
    {
        ensure_storage_object(object_key).await?;
        return Ok(());
    }
    let bytes = minimal_webp_bytes();
    media::insert_file(
        app_pool,
        tenant,
        FileInsert {
            id: file_id,
            entity_type: "PortalBanner".into(),
            entity_id: banner_id,
            bucket: DEV_MEDIA_BUCKET.into(),
            object_key: object_key.into(),
            mime_type: "image/webp".into(),
            size_bytes: bytes.len() as i64,
            sha256: format!("dev-seed-{object_key}"),
            uploaded_by_user_id: admin_user_id(),
        },
    )
    .await?;
    ensure_storage_object(object_key).await?;
    Ok(())
}

async fn ensure_storage_object(object_key: &str) -> DevSeedResult<()> {
    let Some(storage) = open_dev_storage() else {
        return Ok(());
    };
    let bytes = minimal_webp_bytes();
    storage
        .put_object(DEV_MEDIA_BUCKET, object_key, &bytes, "image/webp")
        .await
        .map_err(|err| crate::error::DevSeedError::Aborted(format!("storage put: {err}")))?;
    Ok(())
}

fn open_dev_storage() -> Option<LocalFsObjectStorage> {
    if let Ok(path) = std::env::var("MEDIA_LOCAL_PATH") {
        if let Ok(storage) = LocalFsObjectStorage::new(path) {
            return Some(storage);
        }
    }
    LocalFsObjectStorage::new(".local/object-storage").ok()
}
