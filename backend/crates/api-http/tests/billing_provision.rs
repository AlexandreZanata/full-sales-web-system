//! Provision flow — Asaas customer sync contract tests.

#[path = "support/mod.rs"]
mod support;

use std::sync::Arc;

use http::StatusCode;
use infra_asaas::FailingPaymentGateway;
use serde_json::json;

use support::{platform_access_token, request, seed_platform_admin, setup};

fn env_with_failing_gateway(mut env: support::TestEnv) -> support::TestEnv {
    env.state.payment_gateway = Arc::new(FailingPaymentGateway);
    env
}

#[tokio::test]
async fn contract_provision_when_asaas_fails_then_stays_provisioning_with_dead_letter() {
    let env = env_with_failing_gateway(setup().await);
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;

    let body = json!({
        "legalName": "Failed Asaas Co",
        "displayName": "Failed",
        "adminEmail": "failed-asaas@test.com",
        "planId": "01900002-0001-7000-8000-000000000001",
        "trial": true,
        "cnpj": "11222333000181"
    })
    .to_string();

    let (status, resp) = request(
        &env,
        "POST",
        "/v1/platform/tenants",
        Some(&token),
        Some(body),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED, "provision: {resp}");
    assert_eq!(resp["status"], "Provisioning");

    let tenant_id = domain_shared::TenantId::from_uuid(
        uuid::Uuid::parse_str(resp["tenantId"].as_str().expect("id")).expect("uuid"),
    );
    let customer = infra_postgres::billing::find_asaas_customer_id(&env.admin_pool, tenant_id)
        .await
        .expect("lookup");
    assert!(customer.is_none());

    let count =
        infra_postgres::billing::count_provisioning_dead_letters(&env.admin_pool, tenant_id)
            .await
            .expect("dead letter count");
    assert_eq!(count, 1);
}
