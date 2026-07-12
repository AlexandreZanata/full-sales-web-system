//! Phase 13B — order resource cross-tenant isolation.

use http::StatusCode;
use uuid::Uuid;

use crate::support::{request, seed_admin, seed_commerce, seed_order, setup};

#[tokio::test]
async fn contract_tenant_a_admin_cannot_read_tenant_b_order() {
    let env = setup().await;
    let (admin_id, admin_token) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let own_order = seed_order(&env, commerce_id, admin_id).await;

    let other_tenant = domain_shared::TenantId::generate();
    infra_postgres::shared::insert_tenant(&env.admin_pool, other_tenant, "Other Tenant")
        .await
        .expect("tenant");
    let other_admin = Uuid::now_v7();
    let hash = infra_crypto::PasswordHasher::hash("secret123").expect("hash");
    infra_postgres::identity::insert_user(
        &env.app_pool,
        other_tenant,
        infra_postgres::identity::InsertUserParams {
            id: other_admin,
            email: "other-admin@test.com",
            name: "Other Admin",
            role: "Admin",
            password_hash: &hash,
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("user");
    let other_commerce = Uuid::now_v7();
    infra_postgres::commerces::insert_commerce(
        &env.app_pool,
        other_tenant,
        other_commerce,
        "55443322000190",
        "Other Ltda",
        "Other Store",
        serde_json::json!({}),
    )
    .await
    .expect("commerce");
    let other_order = Uuid::now_v7();
    let other_session = infra_postgres::SessionContext {
        tenant_id: other_tenant,
        role: "Admin".into(),
        user_id: other_admin,
        commerce_id: None,
    };
    let address_id = Uuid::now_v7();
    infra_postgres::commerces::addresses::insert_address(
        &env.app_pool,
        other_tenant,
        infra_postgres::commerces::addresses::AddressInsert {
            id: address_id,
            commerce_id: other_commerce,
            address_type: "Delivery".into(),
            street: "Rua B".into(),
            number: "2".into(),
            district: None,
            city: "RJ".into(),
            state: "RJ".into(),
            postal_code: "01310100".into(),
            latitude: None,
            longitude: None,
            is_primary: true,
        },
    )
    .await
    .expect("address");
    let other_product = Uuid::now_v7();
    infra_postgres::inventory::insert_product_with_catalog(
        &env.app_pool,
        other_tenant,
        infra_postgres::inventory::ProductInsert {
            id: other_product,
            sku: "ORD-OTHER".into(),
            name: "Other Order Product".into(),
            price_amount: 500,
            price_currency: "BRL".into(),
            compare_at_price: None,
            category_id: None,
            unit_of_measure: "Unit".into(),
            description: None,
        },
    )
    .await
    .expect("product");
    infra_postgres::orders::insert_order(
        &env.app_pool,
        &other_session,
        &infra_postgres::orders::OrderInsert {
            id: other_order,
            commerce_id: other_commerce,
            created_by_user_id: other_admin,
            source: "SellerVisit".into(),
            status: "PendingApproval".into(),
            delivery_address_id: address_id,
            notes: None,
            total_amount: 500,
            total_currency: "BRL".into(),
        },
    )
    .await
    .expect("other order");

    let (status, _) = request(
        &env,
        "GET",
        &format!("/v1/orders/{own_order}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/orders/{other_order}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "ORDER_NOT_FOUND");
}
