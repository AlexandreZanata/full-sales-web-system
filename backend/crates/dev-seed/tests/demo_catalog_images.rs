//! Contract: demo seed gives every catalog entity a primary JPEG in storage.

use dev_seed::{SeedPools, seed_dev_dataset};
use infra_postgres::inventory::product_images;
use infra_postgres::portal::{banners, promotions};
use infra_postgres::{connect, migrate};
use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use uuid::Uuid;

async fn setup_pools() -> (
    infra_postgres::PgPool,
    infra_postgres::PgPool,
    testcontainers::ContainerAsync<Postgres>,
) {
    let container = Postgres::default()
        .with_tag("18-alpine")
        .start()
        .await
        .expect("start postgres");

    let host = container.get_host().await.expect("host");
    let port = container.get_host_port_ipv4(5432).await.expect("port");
    let admin_url = format!("postgres://postgres:postgres@{host}:{port}/postgres");
    let app_url = format!("postgres://app_user:app_password@{host}:{port}/postgres");

    let admin = connect(&admin_url).await.expect("admin pool");
    migrate(&admin).await.expect("migrate");
    let app = connect(&app_url).await.expect("app pool");
    (admin, app, container)
}

#[tokio::test]
async fn given_fresh_db_when_seed_then_all_demo_entities_have_images() {
    let (admin, app, _container) = setup_pools().await;
    let pools = SeedPools { admin, app };
    seed_dev_dataset(&pools).await.expect("seed");

    let tenant = dev_seed::ids::tenant_id();

    let products = infra_postgres::inventory::list_products(&pools.app, tenant, None, 50, 0)
        .await
        .expect("products");
    assert_eq!(products.len(), 12, "expected 12 demo products");

    for product in &products {
        let images = product_images::list_product_images(&pools.app, tenant, product.id)
            .await
            .expect("product images");
        assert!(
            images.iter().any(|row| row.is_primary),
            "product {} ({}) missing primary image",
            product.sku,
            product.name
        );
    }

    let categories = infra_postgres::inventory::product_categories::list_categories(
        &pools.app, tenant, None, 20, 0,
    )
    .await
    .expect("categories");
    assert_eq!(categories.len(), 5);
    for category in &categories {
        assert!(
            category.image_file_id.is_some(),
            "category {} missing image",
            category.slug
        );
    }

    let hero_banners = banners::list_banners(&pools.app, tenant, 10)
        .await
        .expect("banners");
    let hero_banners: Vec<_> = hero_banners
        .into_iter()
        .filter(|row| row.placement == "hero" && row.active)
        .collect();
    assert_eq!(hero_banners.len(), 3);
    for banner in &hero_banners {
        assert_ne!(
            banner.image_file_id,
            Uuid::nil(),
            "banner {} missing image",
            banner.id
        );
    }

    let promos = promotions::list_promotions(&pools.app, tenant, 10)
        .await
        .expect("promotions");
    let promos: Vec<_> = promos.into_iter().filter(|row| row.active).collect();
    assert_eq!(promos.len(), 2);
    for promo in &promos {
        assert!(
            promo.image_file_id.is_some(),
            "promotion {} missing image",
            promo.headline
        );
    }
}
