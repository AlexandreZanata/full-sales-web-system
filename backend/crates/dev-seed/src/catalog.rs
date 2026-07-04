use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::inventory::product_images::{self, ProductImageInsert};
use infra_postgres::inventory::{self, ProductInsert, StockMovementInsert};
use infra_postgres::media::{self, FileInsert};
use uuid::Uuid;

use crate::commerces::CommercesSeed;
use crate::error::DevSeedResult;
use crate::ids::{admin_user_id, product_ids};
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
    let specs = [
        ("SEED-001", "Premium Widget", 2_500_00, true),
        ("SEED-002", "Standard Gadget", 1_200_00, true),
        ("SEED-003", "Economy Pack", 450_00, true),
        ("SEED-004", "Legacy Item", 900_00, false),
    ];
    let ids = product_ids();
    for (idx, (sku, name, price, active)) in specs.iter().enumerate() {
        seed_product_if_missing(app_pool, tenant, ids[idx], sku, name, *price).await?;
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
                    category: None,
                    unit_of_measure: None,
                },
            )
            .await?;
        }
    }

    seed_product_images(app_pool, tenant, ids[0], ids[1]).await?;
    seed_stock_and_movements(app_pool, tenant, users, &ids).await?;

    Ok(CatalogSeed {
        product_ids: ids.to_vec(),
    })
}

async fn seed_product_if_missing(
    app_pool: &PgPool,
    tenant: TenantId,
    id: Uuid,
    sku: &str,
    name: &str,
    price: i64,
) -> DevSeedResult<()> {
    if inventory::find_product_by_id(app_pool, tenant, id)
        .await?
        .is_some()
    {
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
            category: Some("General".into()),
            unit_of_measure: "Unit".into(),
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
    Ok(())
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
