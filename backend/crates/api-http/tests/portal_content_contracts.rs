//! Phase 17F — Portal admin content CRUD (T-17-064..071).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{request, seed_admin, seed_driver, setup};

// T-17-064 / T-17-065 / T-17-066 / T-17-067
#[tokio::test]
async fn given_admin_when_banner_crud_then_ok() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/portal/banners",
        Some(&admin),
        Some(
            json!({
                "placement": "hero",
                "imageUrl": "https://cdn.example.com/hero.webp",
                "altText": "Hero",
                "sortOrder": 1,
                "active": true
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let id = created["id"].as_str().expect("id");

    let (list_status, list) = request(
        &env,
        "GET",
        "/v1/portal/banners?limit=50",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(
        list["data"]
            .as_array()
            .expect("data")
            .iter()
            .any(|b| b["id"] == id)
    );

    let (patch_status, patched) = request(
        &env,
        "PATCH",
        &format!("/v1/portal/banners/{id}"),
        Some(&admin),
        Some(json!({ "altText": "Hero Updated", "active": false }).to_string()),
    )
    .await;
    assert_eq!(patch_status, StatusCode::OK);
    assert_eq!(patched["altText"], "Hero Updated");

    let (del_status, _) = request(
        &env,
        "DELETE",
        &format!("/v1/portal/banners/{id}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(del_status, StatusCode::NO_CONTENT);

    let (missing_status, missing) = request(
        &env,
        "DELETE",
        &format!("/v1/portal/banners/{}", Uuid::now_v7()),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(missing_status, StatusCode::NOT_FOUND);
    assert_eq!(missing["error"]["code"], "BANNER_NOT_FOUND");
}

// T-17-068 / T-17-069 / T-17-070 / T-17-071
#[tokio::test]
async fn given_admin_when_promotion_crud_then_ok() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/portal/promotions",
        Some(&admin),
        Some(
            json!({
                "headline": "Deal",
                "discountText": "10% OFF",
                "background": "yellow",
                "active": true
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let id = created["id"].as_str().expect("id");

    let (list_status, list) = request(
        &env,
        "GET",
        "/v1/portal/promotions?limit=50",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(
        list["data"]
            .as_array()
            .expect("data")
            .iter()
            .any(|p| p["id"] == id)
    );

    let (patch_status, patched) = request(
        &env,
        "PATCH",
        &format!("/v1/portal/promotions/{id}"),
        Some(&admin),
        Some(json!({ "headline": "Bigger Deal" }).to_string()),
    )
    .await;
    assert_eq!(patch_status, StatusCode::OK);
    assert_eq!(patched["headline"], "Bigger Deal");

    let (del_status, _) = request(
        &env,
        "DELETE",
        &format!("/v1/portal/promotions/{id}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(del_status, StatusCode::NO_CONTENT);

    let (missing_status, missing) = request(
        &env,
        "PATCH",
        &format!("/v1/portal/promotions/{}", Uuid::now_v7()),
        Some(&admin),
        Some(json!({ "headline": "x" }).to_string()),
    )
    .await;
    assert_eq!(missing_status, StatusCode::NOT_FOUND);
    assert_eq!(missing["error"]["code"], "PROMOTION_NOT_FOUND");
}

// T-17-064 / T-17-065 authz
#[tokio::test]
async fn given_driver_when_portal_content_then_403() {
    let env = setup().await;
    let (_, driver) = seed_driver(&env, "driver-banner@test.com").await;
    let (status, body) = request(&env, "GET", "/v1/portal/banners", Some(&driver), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");
}
