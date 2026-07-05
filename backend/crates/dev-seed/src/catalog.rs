use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::inventory::product_categories::{self, CategoryInsert};
use infra_postgres::inventory::product_images::{self, ProductImageInsert};
use infra_postgres::inventory::{self, ProductInsert, StockMovementInsert};
use infra_postgres::media::{self, FileInsert};
use infra_storage::{LocalFsObjectStorage, ObjectStorage};
use uuid::Uuid;

use crate::commerces::CommercesSeed;
use crate::error::DevSeedResult;
use crate::ids::{admin_user_id, category_ids, category_image_file_ids, product_ids};
use crate::media_bytes::{DEV_MEDIA_BUCKET, minimal_webp_bytes};
use crate::users::UsersSeed;

pub struct CatalogSeed {
    pub product_ids: Vec<Uuid>,
}

pub async fn seed_catalog(
    app_pool: &PgPool,
    tenant: TenantId,
    users: &UsersSeed,
    _commerces: &CommercesSeed,
) -> DevSeedResult<CatalogSeed> {
    let categories = category_ids();
    seed_categories(app_pool, tenant, &categories).await?;
    seed_category_images(app_pool, tenant, &categories).await?;

    let specs = [
        ("SEED-001", "Refrigerante Cola 2L", 8_50, true, categories[0]),
        ("SEED-002", "Batata Chips Original", 12_00, true, categories[1]),
        ("SEED-003", "Detergente Neutro 500ml", 14_50, true, categories[2]),
        ("SEED-004", "Pizza Congelada Mussarela", 29_00, false, categories[3]),
    ];
    let ids = product_ids();
    for (idx, (sku, name, price, active, category_id)) in specs.iter().enumerate() {
        seed_product_if_missing(app_pool, tenant, ids[idx], sku, name, *price, *category_id).await?;
        if !active {
            inventory::update_product(
                app_pool,
                tenant,
                ids[idx],
                &inventory::ProductUpdate {
                    name: None,
                    price_amount: None,
                    price_currency: None,
                    active: Some(false),
                    category_id: None,
                    unit_of_measure: None,
                    description: None,
                },
            )
            .await?;
        }
    }

    seed_product_images(app_pool, tenant, ids[0], ids[1]).await?;
    ensure_catalog_storage_objects().await?;
    seed_stock_and_movements(app_pool, tenant, users, &ids).await?;

    Ok(CatalogSeed {
        product_ids: ids.to_vec(),
    })
}

async fn seed_categories(
    app_pool: &PgPool,
    tenant: TenantId,
    ids: &[Uuid; 5],
) -> DevSeedResult<()> {
    let specs = [
        (ids[0], "Bebidas", "bebidas", 0),
        (ids[1], "Snacks", "snacks", 1),
        (ids[2], "Limpeza", "limpeza", 2),
        (ids[3], "Congelados", "congelados", 3),
        (ids[4], "Mercearia", "mercearia", 4),
    ];
    for (default_id, name, slug, sort_order) in specs {
        if let Some(existing) =
            product_categories::find_category_by_slug(app_pool, tenant, slug).await?
        {
            product_categories::update_category(
                app_pool,
                tenant,
                existing.id,
                &product_categories::CategoryUpdate {
                    name: Some(name.into()),
                    slug: None,
                    description: None,
                    sort_order: Some(sort_order),
                    active: Some(true),
                    image_file_id: None,
                },
            )
            .await?;
            continue;
        }
        if product_categories::find_category_by_id(app_pool, tenant, default_id)
            .await?
            .is_some()
        {
            continue;
        }
        product_categories::insert_category(
            app_pool,
            tenant,
            CategoryInsert {
                id: default_id,
                name: name.into(),
                slug: slug.into(),
                description: None,
                sort_order,
                active: true,
                image_file_id: None,
            },
        )
        .await?;
    }
    Ok(())
}

async fn seed_category_images(
    app_pool: &PgPool,
    tenant: TenantId,
    category_ids: &[Uuid; 5],
) -> DevSeedResult<()> {
    let file_ids = category_image_file_ids();
    let keys = [
        "categories/bebidas.webp",
        "categories/snacks.webp",
        "categories/limpeza.webp",
    ];
    for (idx, object_key) in keys.iter().enumerate() {
        let category_id = category_ids[idx];
        let file_id = file_ids[idx];
        insert_category_image_file(app_pool, tenant, file_id, category_id, object_key).await?;
        if let Some(row) =
            product_categories::find_category_by_id(app_pool, tenant, category_id).await?
        {
            if row.image_file_id.is_none() {
                product_categories::update_category(
                    app_pool,
                    tenant,
                    category_id,
                    &product_categories::CategoryUpdate {
                        name: None,
                        slug: None,
                        description: None,
                        sort_order: None,
                        active: None,
                        image_file_id: Some(Some(file_id)),
                    },
                )
                .await?;
            }
        }
    }
    Ok(())
}

async fn insert_category_image_file(
    app_pool: &PgPool,
    tenant: TenantId,
    file_id: Uuid,
    category_id: Uuid,
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
            entity_type: "ProductCategory".into(),
            entity_id: category_id,
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

async fn seed_product_if_missing(
    app_pool: &PgPool,
    tenant: TenantId,
    id: Uuid,
    sku: &str,
    name: &str,
    price: i64,
    category_id: Uuid,
) -> DevSeedResult<()> {
    if inventory::find_product_by_id(app_pool, tenant, id)
        .await?
        .is_some()
    {
        inventory::update_product(
            app_pool,
            tenant,
            id,
            &inventory::ProductUpdate {
                name: None,
                price_amount: None,
                price_currency: None,
                active: None,
                category_id: Some(Some(category_id)),
                unit_of_measure: None,
                description: None,
            },
        )
        .await?;
        return Ok(());
    }
    inventory::insert_product_with_catalog(
        app_pool,
        tenant,
        ProductInsert {
            id,
            sku: sku.into(),
            name: name.into(),
            price_amount: price,
            price_currency: "BRL".into(),
            category_id: Some(category_id),
            unit_of_measure: "Unit".into(),
            description: None,
        },
    )
    .await?;
    Ok(())
}

async fn seed_product_images(
    app_pool: &PgPool,
    tenant: TenantId,
    product_a: Uuid,
    product_b: Uuid,
) -> DevSeedResult<()> {
    let file_a = Uuid::parse_str("01900001-0021-7000-8000-000000000001").expect("img a");
    let file_b = Uuid::parse_str("01900001-0021-7000-8000-000000000002").expect("img b");
    insert_image_file(
        app_pool,
        tenant,
        file_a,
        product_a,
        "products/widget-a.webp",
    )
    .await?;
    insert_image_file(
        app_pool,
        tenant,
        file_b,
        product_b,
        "products/gadget-b.webp",
    )
    .await?;

    let image_a = Uuid::parse_str("01900001-0022-7000-8000-000000000001").expect("pi a");
    let image_b = Uuid::parse_str("01900001-0022-7000-8000-000000000002").expect("pi b");
    insert_product_image_row(app_pool, tenant, image_a, product_a, file_a, 0, true).await?;
    insert_product_image_row(app_pool, tenant, image_b, product_b, file_b, 0, true).await?;
    Ok(())
}

async fn insert_image_file(
    app_pool: &PgPool,
    tenant: TenantId,
    file_id: Uuid,
    product_id: Uuid,
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
            entity_type: "Product".into(),
            entity_id: product_id,
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
        .put_object(
            DEV_MEDIA_BUCKET,
            object_key,
            &bytes,
            "image/webp",
        )
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

async fn insert_product_image_row(
    app_pool: &PgPool,
    tenant: TenantId,
    id: Uuid,
    product_id: Uuid,
    file_id: Uuid,
    sort_order: i32,
    is_primary: bool,
) -> DevSeedResult<()> {
    if product_images::find_product_image_by_id(app_pool, tenant, id)
        .await?
        .is_some()
    {
        return Ok(());
    }
    product_images::insert_product_image(
        app_pool,
        tenant,
        ProductImageInsert {
            id,
            product_id,
            file_id,
            sort_order,
            is_primary,
        },
    )
    .await?;
    Ok(())
}

/// Idempotent backfill for categories + product assignments on existing dev DBs.
pub async fn ensure_catalog_categories(app_pool: &PgPool, tenant: TenantId) -> DevSeedResult<()> {
    let categories = category_ids();
    seed_categories(app_pool, tenant, &categories).await?;
    seed_category_images(app_pool, tenant, &categories).await?;

    let specs = [
        ("SEED-001", categories[0]),
        ("SEED-002", categories[1]),
        ("SEED-003", categories[2]),
        ("SEED-004", categories[3]),
    ];
    let ids = product_ids();
    for (idx, (_, category_id)) in specs.iter().enumerate() {
        if inventory::find_product_by_id(app_pool, tenant, ids[idx])
            .await?
            .is_some()
        {
            inventory::update_product(
                app_pool,
                tenant,
                ids[idx],
                &inventory::ProductUpdate {
                    name: None,
                    price_amount: None,
                    price_currency: None,
                    active: None,
                    category_id: Some(Some(*category_id)),
                    unit_of_measure: None,
                    description: None,
                },
            )
            .await?;
        }
    }
    Ok(())
}

/// Idempotent backfill for local object storage (safe when DB seed was applied earlier).
pub async fn ensure_catalog_storage_objects() -> DevSeedResult<()> {
    ensure_storage_object("products/widget-a.webp").await?;
    ensure_storage_object("products/gadget-b.webp").await?;
    ensure_storage_object("categories/bebidas.webp").await?;
    ensure_storage_object("categories/snacks.webp").await?;
    ensure_storage_object("categories/limpeza.webp").await?;
    Ok(())
}

async fn seed_stock_and_movements(
    app_pool: &PgPool,
    tenant: TenantId,
    users: &UsersSeed,
    product_ids: &[Uuid],
) -> DevSeedResult<()> {
    let drivers = [users.driver_a_id, users.driver_b_id];
    for driver in drivers {
        for product_id in product_ids.iter().take(3) {
            inventory::upsert_stock_balance(app_pool, tenant, driver, *product_id, 40).await?;
        }
    }

    let movements = [
        (
            Uuid::parse_str("01900001-0023-7000-8000-000000000001").expect("m1"),
            product_ids[0],
            users.driver_a_id,
            "Inbound",
            20,
            Some("Initial load".into()),
        ),
        (
            Uuid::parse_str("01900001-0023-7000-8000-000000000002").expect("m2"),
            product_ids[1],
            users.driver_a_id,
            "Adjustment",
            5,
            Some("Route prep".into()),
        ),
        (
            Uuid::parse_str("01900001-0023-7000-8000-000000000003").expect("m3"),
            product_ids[0],
            users.driver_b_id,
            "Inbound",
            15,
            None,
        ),
        (
            Uuid::parse_str("01900001-0023-7000-8000-000000000004").expect("m4"),
            product_ids[2],
            users.driver_b_id,
            "Inbound",
            10,
            Some("Restock".into()),
        ),
        (
            Uuid::parse_str("01900001-0023-7000-8000-000000000005").expect("m5"),
            product_ids[1],
            users.driver_b_id,
            "Adjustment",
            3,
            Some("Damaged units".into()),
        ),
    ];
    for (id, product_id, driver_id, movement_type, qty, reason) in movements {
        if inventory::list_stock_movements_by_product(app_pool, tenant, product_id, 100, 0)
            .await?
            .iter()
            .any(|row| row.id == id)
        {
            continue;
        }
        inventory::insert_stock_movement(
            app_pool,
            tenant,
            StockMovementInsert {
                id,
                product_id,
                responsible_id: driver_id,
                movement_type: movement_type.into(),
                quantity: qty,
                reference_id: None,
                reason,
            },
        )
        .await?;
    }
    Ok(())
}
