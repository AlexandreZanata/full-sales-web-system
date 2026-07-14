//! Phase 17D — Categories CRUD contracts (T-17-030..036).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{minimal_webp_bytes, request, seed_admin, seed_driver, setup, upload_multipart};

async fn create_category(
    env: &support::TestEnv,
    admin: &str,
    name: &str,
) -> (StatusCode, serde_json::Value) {
    request(
        env,
        "POST",
        "/v1/categories",
        Some(admin),
        Some(json!({ "name": name }).to_string()),
    )
    .await
}

// T-17-031 / T-17-030 / T-17-032
#[tokio::test]
async fn given_admin_when_category_crud_happy_then_ok() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;

    let (create_status, created) = create_category(&env, &admin, "Beverages").await;
    assert_eq!(create_status, StatusCode::CREATED);
    let id = created["id"].as_str().expect("id");
    assert_eq!(created["name"], "Beverages");

    let (list_status, list) =
        request(&env, "GET", "/v1/categories?limit=20", Some(&admin), None).await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(
        list["data"]
            .as_array()
            .expect("data")
            .iter()
            .any(|c| c["id"] == id)
    );

    let (get_status, got) = request(
        &env,
        "GET",
        &format!("/v1/categories/{id}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(get_status, StatusCode::OK);
    assert_eq!(got["id"], id);
}

// T-17-032
#[tokio::test]
async fn given_unknown_category_when_get_then_404() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let missing = Uuid::now_v7();
    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/categories/{missing}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "CATEGORY_NOT_FOUND");
}

// T-17-033 / T-17-034
#[tokio::test]
async fn given_admin_when_patch_and_delete_category_then_ok() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (_, created) = create_category(&env, &admin, "Snacks").await;
    let id = created["id"].as_str().expect("id");

    let (patch_status, patched) = request(
        &env,
        "PATCH",
        &format!("/v1/categories/{id}"),
        Some(&admin),
        Some(json!({ "name": "Snacks Updated", "active": true }).to_string()),
    )
    .await;
    assert_eq!(patch_status, StatusCode::OK);
    assert_eq!(patched["name"], "Snacks Updated");

    let (del_status, _) = request(
        &env,
        "DELETE",
        &format!("/v1/categories/{id}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(del_status, StatusCode::NO_CONTENT);
}

// T-17-035 / T-17-036
#[tokio::test]
async fn given_admin_when_reorder_and_image_then_ok() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (_, a) = create_category(&env, &admin, "Cat A").await;
    let (_, b) = create_category(&env, &admin, "Cat B").await;
    let id_a = a["id"].as_str().expect("a");
    let id_b = b["id"].as_str().expect("b");

    let (reorder_status, _) = request(
        &env,
        "POST",
        "/v1/categories/reorder",
        Some(&admin),
        Some(json!({ "orderedIds": [id_b, id_a] }).to_string()),
    )
    .await;
    assert_eq!(reorder_status, StatusCode::NO_CONTENT);

    let webp = minimal_webp_bytes();
    let cat_uuid = Uuid::parse_str(id_a).expect("uuid");
    let (upload_status, upload) = upload_multipart(
        &env,
        &admin,
        "cat.webp",
        "image/webp",
        &webp,
        "ProductCategory",
        cat_uuid,
    )
    .await;
    assert!(upload_status.is_success(), "{upload}");
    let file_id = upload["id"].as_str().expect("file");

    let (img_status, img) = request(
        &env,
        "PUT",
        &format!("/v1/categories/{id_a}/image"),
        Some(&admin),
        Some(json!({ "fileId": file_id }).to_string()),
    )
    .await;
    assert_eq!(img_status, StatusCode::OK);
    assert!(img.get("thumbUrl").is_some() || img.get("imageFileId").is_some());
}

// T-17-030 / T-17-031 authz
#[tokio::test]
async fn given_driver_when_categories_then_403() {
    let env = setup().await;
    let (_, driver) = seed_driver(&env, "driver-cat-authz@test.com").await;
    let (list_status, list_body) =
        request(&env, "GET", "/v1/categories", Some(&driver), None).await;
    assert_eq!(list_status, StatusCode::FORBIDDEN);
    assert_eq!(list_body["error"]["code"], "FORBIDDEN");

    let (post_status, _) = request(
        &env,
        "POST",
        "/v1/categories",
        Some(&driver),
        Some(json!({ "name": "Nope" }).to_string()),
    )
    .await;
    assert_eq!(post_status, StatusCode::FORBIDDEN);
}
