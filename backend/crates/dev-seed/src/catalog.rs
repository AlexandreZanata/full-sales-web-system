use domain_shared::TenantId;
use infra_postgres::PgPool;
use infra_postgres::inventory::product_categories::{self, CategoryInsert, CategoryUpdate};
use infra_postgres::inventory::{self, StockMovementInsert};
use uuid::Uuid;

use crate::commerces::CommercesSeed;
use crate::demo_catalog;
use crate::error::DevSeedResult;
use crate::ids::{category_ids, product_ids};
use crate::users::UsersSeed;

pub struct CatalogSeed {
    pub product_ids: Vec<Uuid>,
}

pub async fn seed_catalog(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
    users: &UsersSeed,
    _commerces: &CommercesSeed,
) -> DevSeedResult<CatalogSeed> {
    let categories = category_ids();
    seed_categories(app_pool, tenant, &categories).await?;
    demo_catalog::ensure_demo_catalog(app_pool, admin_pool, tenant).await?;
    seed_stock_and_movements(app_pool, tenant, users, &product_ids()).await?;

    Ok(CatalogSeed {
        product_ids: product_ids().to_vec(),
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
                &CategoryUpdate {
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

/// Idempotent backfill for categories + demo catalog on existing dev DBs.
pub async fn ensure_catalog_categories(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
) -> DevSeedResult<()> {
    let categories = category_ids();
    seed_categories(app_pool, tenant, &categories).await?;
    demo_catalog::ensure_demo_catalog(app_pool, admin_pool, tenant).await
}

async fn seed_stock_and_movements(
    app_pool: &PgPool,
    tenant: TenantId,
    users: &UsersSeed,
    product_ids: &[Uuid],
) -> DevSeedResult<()> {
    let drivers = [users.driver_a_id, users.driver_b_id];
    for driver in drivers {
        for product_id in product_ids.iter().take(8) {
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
            product_ids[4],
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
            product_ids[7],
            users.driver_b_id,
            "Inbound",
            10,
            Some("Restock".into()),
        ),
        (
            Uuid::parse_str("01900001-0023-7000-8000-000000000005").expect("m5"),
            product_ids[5],
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
