//! Cross-tenant seed helpers for isolation tests.

use uuid::Uuid;

pub async fn seed_other_tenant_commerce(
    env: &crate::support::TestEnv,
) -> (domain_shared::TenantId, Uuid) {
    let other_tenant = domain_shared::TenantId::generate();
    infra_postgres::shared::insert_tenant(&env.admin_pool, other_tenant, "Other Tenant")
        .await
        .expect("other tenant");
    let other_commerce = Uuid::now_v7();
    infra_postgres::commerces::insert_commerce(
        &env.app_pool,
        other_tenant,
        other_commerce,
        "99888777000161",
        "Other Ltda",
        "Other Store",
        serde_json::json!({"city": "RJ"}),
    )
    .await
    .expect("other commerce");
    (other_tenant, other_commerce)
}

pub async fn seed_other_tenant_product(env: &crate::support::TestEnv) -> Uuid {
    let other_tenant = domain_shared::TenantId::generate();
    infra_postgres::shared::insert_tenant(&env.admin_pool, other_tenant, "Other Tenant")
        .await
        .expect("other tenant");
    let product_id = Uuid::now_v7();
    infra_postgres::inventory::insert_product_with_catalog(
        &env.app_pool,
        other_tenant,
        infra_postgres::inventory::ProductInsert {
            id: product_id,
            sku: "OTHER-SKU".into(),
            name: "Other Product".into(),
            price_amount: 900,
            price_currency: "BRL".into(),
            compare_at_price: None,
            category_id: None,
            unit_of_measure: "Unit".into(),
            description: None,
        },
    )
    .await
    .expect("other product");
    product_id
}
