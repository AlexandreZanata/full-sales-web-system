//! Expanded demo catalog — 12 products with realistic names and bundled photos.

use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::inventory::product_categories::{self, CategoryUpdate};
use infra_postgres::inventory::{self, ProductInsert, ProductUpdate};
use uuid::Uuid;

use crate::demo_products::{CATEGORY_ASSETS, DemoProduct, demo_products};
use crate::error::DevSeedResult;
use crate::ids::{
    admin_user_id, category_ids, category_image_file_ids, product_image_file_ids,
    product_image_row_ids,
};
use crate::seed_assets::{ensure_media_file, ensure_product_image_link, ensure_storage_bytes};

pub async fn ensure_demo_catalog(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
) -> DevSeedResult<()> {
    seed_demo_products(app_pool, tenant).await?;
    seed_demo_product_images(app_pool, admin_pool, tenant).await?;
    seed_demo_category_images(app_pool, admin_pool, tenant).await?;
    ensure_demo_storage_objects().await?;
    Ok(())
}

async fn seed_demo_products(app_pool: &PgPool, tenant: TenantId) -> DevSeedResult<()> {
    let categories = category_ids();
    for product in demo_products() {
        let category_id = categories[product.category_idx];
        upsert_demo_product(app_pool, tenant, &product, category_id).await?;
    }
    Ok(())
}

async fn upsert_demo_product(
    app_pool: &PgPool,
    tenant: TenantId,
    product: &DemoProduct,
    category_id: Uuid,
) -> DevSeedResult<()> {
    if inventory::find_product_by_id(app_pool, tenant, product.id)
        .await?
        .is_some()
    {
        inventory::update_product(
            app_pool,
            tenant,
            product.id,
            &ProductUpdate {
                name: Some(product.name.into()),
                price_amount: Some(product.price),
                price_currency: Some("BRL".into()),
                active: Some(product.active),
                category_id: Some(Some(category_id)),
                unit_of_measure: Some(product.unit.into()),
                description: Some(Some(product.description.into())),
                is_featured: None,
            },
        )
        .await?;
        return Ok(());
    }
    inventory::insert_product_with_catalog(
        app_pool,
        tenant,
        ProductInsert {
            id: product.id,
            sku: product.sku.into(),
            name: product.name.into(),
            price_amount: product.price,
            price_currency: "BRL".into(),
            category_id: Some(category_id),
            unit_of_measure: product.unit.into(),
            description: Some(product.description.into()),
        },
    )
    .await?;
    if !product.active {
        inventory::update_product(
            app_pool,
            tenant,
            product.id,
            &ProductUpdate {
                name: None,
                price_amount: None,
                price_currency: None,
                active: Some(false),
                category_id: None,
                unit_of_measure: None,
                description: None,
                is_featured: None,
            },
        )
        .await?;
    }
    Ok(())
}

async fn seed_demo_product_images(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
) -> DevSeedResult<()> {
    let file_ids = product_image_file_ids();
    let row_ids = product_image_row_ids();
    let uploader = admin_user_id();
    let products = demo_products();

    for (index, product) in products.iter().enumerate() {
        ensure_media_file(
            app_pool,
            admin_pool,
            tenant,
            file_ids[index],
            "Product",
            product.id,
            product.object_key,
            product.asset,
            uploader,
        )
        .await?;
        ensure_product_image_link(
            app_pool,
            tenant,
            row_ids[index],
            product.id,
            file_ids[index],
        )
        .await?;
    }
    Ok(())
}

async fn seed_demo_category_images(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
) -> DevSeedResult<()> {
    let categories = category_ids();
    let file_ids = category_image_file_ids();
    let uploader = admin_user_id();

    for (index, (asset, object_key)) in CATEGORY_ASSETS.iter().enumerate() {
        let category_id = categories[index];
        ensure_media_file(
            app_pool,
            admin_pool,
            tenant,
            file_ids[index],
            "ProductCategory",
            category_id,
            object_key,
            asset,
            uploader,
        )
        .await?;
        link_category_image(app_pool, tenant, category_id, file_ids[index]).await?;
    }
    Ok(())
}

async fn link_category_image(
    app_pool: &PgPool,
    tenant: TenantId,
    category_id: Uuid,
    file_id: Uuid,
) -> DevSeedResult<()> {
    if product_categories::find_category_by_id(app_pool, tenant, category_id)
        .await?
        .is_some_and(|row| row.image_file_id == Some(file_id))
    {
        return Ok(());
    }
    product_categories::update_category(
        app_pool,
        tenant,
        category_id,
        &CategoryUpdate {
            name: None,
            slug: None,
            description: None,
            sort_order: None,
            active: None,
            image_file_id: Some(Some(file_id)),
        },
    )
    .await?;
    Ok(())
}

pub async fn ensure_demo_storage_objects() -> DevSeedResult<()> {
    use crate::seed_assets::read_asset_or_placeholder;

    for product in demo_products() {
        let (bytes, mime) = read_asset_or_placeholder(product.asset);
        ensure_storage_bytes(product.object_key, &bytes, mime).await?;
    }
    for (asset, object_key) in CATEGORY_ASSETS {
        let (bytes, mime) = read_asset_or_placeholder(asset);
        ensure_storage_bytes(object_key, &bytes, mime).await?;
    }
    Ok(())
}
