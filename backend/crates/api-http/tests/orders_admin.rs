//! Phase 21 — Admin orders contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;

use support::{request, seed_admin, seed_commerce, seed_commerce_contact, seed_order, setup};

// Contract: admin lists orders across commerces
#[tokio::test]
async fn contract_admin_when_list_orders_then_pagination_with_items() {
    let env = setup().await;
    let (admin_id, admin_token) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let order_id = seed_order(&env, commerce_id, admin_id).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/orders",
        Some(&admin_token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["items"].is_array());
    assert_eq!(body["page"], 1);
    assert_eq!(body["pageSize"], 20);
    assert!(body["total"].as_u64().is_some());
    assert!(body["items"]
        .as_array()
        .unwrap()
        .iter()
        .any(|o| o["id"] == order_id.to_string()));
}

// Contract: commerce contact cannot GET /v1/orders → 403
#[tokio::test]
async fn contract_commerce_contact_when_get_orders_then_forbidden() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (_, contact_token) =
        seed_commerce_contact(&env, commerce_id, "portal@store.com").await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/orders",
        Some(&contact_token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");
}
