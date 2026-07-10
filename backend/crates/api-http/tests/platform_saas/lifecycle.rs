//! Contract: provision → trial → pay → active → suspend → reactivate (Phase 13A).

use chrono::{Duration, Utc};
use domain_billing::SubscriptionStatus;
use domain_platform::TenantStatus;
use http::StatusCode;
use serde_json::json;

use crate::helpers::{
    STARTER_PLAN, payment_confirmed_event, payment_overdue_event, post_asaas_webhook,
};
use crate::support::{
    platform_access_token, request, seed_commerce, seed_driver, seed_driver_stock,
    seed_platform_admin, seed_product, setup,
};

#[tokio::test]
async fn contract_platform_saas_journey_provision_trial_pay_active_suspend_reactivate() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let platform_token = platform_access_token(&env).await;

    let provision_body = json!({
        "legalName": "Journey Co LTDA",
        "displayName": "Journey Store",
        "adminEmail": "journey-admin@test.com",
        "planId": STARTER_PLAN,
        "trial": true,
        "cnpj": "11222333000181"
    })
    .to_string();
    let (status, provisioned) = request(
        &env,
        "POST",
        "/v1/platform/tenants",
        Some(&platform_token),
        Some(provision_body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{provisioned}");
    assert_eq!(provisioned["status"], "Trial");
    let tenant_id = provisioned["tenantId"].as_str().expect("tenantId");
    let tenant_uuid = uuid::Uuid::parse_str(tenant_id).expect("uuid");
    let tenant = domain_shared::TenantId::from_uuid(tenant_uuid);

    let webhook_body = payment_confirmed_event("evt_journey_pay", tenant_id);
    let (status, _) = post_asaas_webhook(&env, webhook_body).await;
    assert_eq!(status, StatusCode::OK);

    let lifecycle = infra_postgres::shared::find_tenant_lifecycle(&env.admin_pool, tenant)
        .await
        .expect("lifecycle")
        .expect("row");
    assert_eq!(lifecycle.status, TenantStatus::Active);

    let sub = infra_postgres::billing::find_subscription_by_tenant(&env.admin_pool, tenant)
        .await
        .expect("sub")
        .expect("subscription");
    assert_eq!(sub.status, SubscriptionStatus::Active);

    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "JRN-1", "Journey Product", 500).await;
    let (driver_id, _) = seed_driver(&env, "journey-driver@test.com").await;
    seed_driver_stock(&env, driver_id, product_id, 5).await;
    let (_, admin_token) = crate::support::seed_admin(&env).await;
    let seeded_tenant_path = format!("/v1/platform/tenants/{}", env.tenant_id.as_uuid());

    let (status, _) = request(
        &env,
        "POST",
        &format!("{seeded_tenant_path}/suspend"),
        Some(&platform_token),
        Some(json!({ "reason": "PO acceptance suspend test" }).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let sale_body = json!({
        "commerceId": commerce_id,
        "driverId": driver_id,
        "items": [{ "productId": product_id, "quantity": 1 }],
        "paymentMethod": "cash"
    })
    .to_string();
    let (status, blocked) = request(
        &env,
        "POST",
        "/v1/sales",
        Some(&admin_token),
        Some(sale_body.clone()),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(blocked["error"]["code"], "TENANT_SUSPENDED");

    let (status, _) = request(
        &env,
        "POST",
        &format!("{seeded_tenant_path}/reactivate"),
        Some(&platform_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, sale) = request(
        &env,
        "POST",
        "/v1/sales",
        Some(&admin_token),
        Some(sale_body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{sale}");
}

#[tokio::test]
async fn contract_platform_saas_past_due_dunning_then_suspended() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let platform_token = platform_access_token(&env).await;

    let body = json!({
        "legalName": "Dunning Co",
        "displayName": "Dunning",
        "adminEmail": "dunning@test.com",
        "planId": STARTER_PLAN,
        "trial": false,
        "cnpj": "11222333000181"
    })
    .to_string();
    let (status, resp) = request(
        &env,
        "POST",
        "/v1/platform/tenants",
        Some(&platform_token),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{resp}");
    let tenant_id = resp["tenantId"].as_str().expect("id");

    let overdue = payment_overdue_event("evt_journey_overdue", tenant_id);
    let (status, _) = post_asaas_webhook(&env, overdue).await;
    assert_eq!(status, StatusCode::OK);

    let tenant_uuid = uuid::Uuid::parse_str(tenant_id).expect("uuid");
    infra_postgres::shared::backdate_past_due_at(
        &env.admin_pool,
        domain_shared::TenantId::from_uuid(tenant_uuid),
        Utc::now() - Duration::days(8),
    )
    .await
    .expect("backdate");

    let (status, dunning) = request(
        &env,
        "POST",
        "/v1/platform/jobs/dunning",
        Some(&platform_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{dunning}");

    let tenant = infra_postgres::shared::find_tenant_lifecycle(
        &env.admin_pool,
        domain_shared::TenantId::from_uuid(tenant_uuid),
    )
    .await
    .expect("tenant")
    .expect("row");
    assert_eq!(tenant.status, TenantStatus::Suspended);
}
