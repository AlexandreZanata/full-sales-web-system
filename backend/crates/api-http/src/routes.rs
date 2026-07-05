use axum::{
    Json, Router,
    routing::{get, patch, post, put},
};
use serde::Serialize;

use crate::admin_orders::{
    approve_order, cancel_order, get_order, list_orders, reject_order_handler, start_picking,
};
use crate::audit::list_audit_events;
use crate::auth::{auth_middleware, login, logout, refresh};
use crate::commerces::{
    create_address, create_commerce, deactivate_commerce, get_commerce, list_addresses,
    list_commerces, update_address, update_logo,
};
use crate::deliveries::{
    confirm_delivery, create_order_delivery, get_delivery, list_deliveries, start_delivery_transit,
};
use crate::inventory::{get_stock_balance, list_movements, list_stock_balances, record_movement};
use crate::media::{get_media_content, get_media_url, get_public_product_media_content, upload_media};
use crate::catalog_events::stream_catalog_events;
use crate::categories::{
    create_category, delete_category, get_category, list_categories, reorder_categories,
    update_category, update_category_image,
};
use crate::portal::{
    cancel_portal_order, create_portal_order, get_portal_category_by_slug, get_portal_order,
    get_public_category_by_slug, list_portal_categories, list_portal_orders, list_portal_products,
    list_public_categories, list_public_products, submit_portal_order, update_portal_order,
};
use crate::products::{
    attach_product_image, create_product, delete_product_image, get_product, list_product_images,
    list_products, update_product,
};
use crate::reports::{export_report, generate_report, get_report, list_reports, verify_report};
use crate::sales::{
    cancel_sale, confirm_sale, create_sale, declare_sale_payment, get_sale, list_sales,
};
use crate::settings::{get_settings, patch_settings, update_site_logo};
use crate::state::AppState;
use crate::users::{
    create_user, deactivate_user, get_user, list_users, upsert_driver_profile,
    upsert_seller_profile,
};

/// API contract: `GET /health` → `{ "status": "ok" }`.
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

/// `GET /v1/` version stub (reserved for future version metadata).
#[derive(Serialize)]
pub struct V1RootResponse {
    pub version: &'static str,
    pub status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn v1_root() -> Json<V1RootResponse> {
    Json(V1RootResponse {
        version: "1",
        status: "ok",
    })
}

pub fn health_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/", get(v1_root))
}

pub fn v1_router(state: AppState) -> Router {
    let public = Router::new()
        .route("/v1/auth/login", post(login))
        .route("/v1/auth/refresh", post(refresh))
        .route("/v1/public/products", get(list_public_products))
        .route("/v1/public/categories", get(list_public_categories))
        .route(
            "/v1/public/categories/{slug}",
            get(get_public_category_by_slug),
        )
        .route("/v1/public/media/{id}/content", get(get_public_product_media_content))
        .route("/v1/public/catalog/events", get(stream_catalog_events))
        .route("/v1/reports/{id}/verify", get(verify_report))
        .with_state(state.clone());

    let protected = Router::new()
        .route("/v1/auth/logout", post(logout))
        .route("/v1/users", post(create_user).get(list_users))
        .route("/v1/users/{id}", get(get_user))
        .route("/v1/users/{id}/deactivate", patch(deactivate_user))
        .route("/v1/users/{id}/driver-profile", put(upsert_driver_profile))
        .route("/v1/users/{id}/seller-profile", put(upsert_seller_profile))
        .route("/v1/commerces", post(create_commerce).get(list_commerces))
        .route("/v1/commerces/{id}", get(get_commerce))
        .route("/v1/commerces/{id}/deactivate", patch(deactivate_commerce))
        .route(
            "/v1/commerces/{id}/addresses",
            get(list_addresses).post(create_address),
        )
        .route(
            "/v1/commerces/{id}/addresses/{addressId}",
            patch(update_address),
        )
        .route("/v1/commerces/{id}/logo", put(update_logo))
        .route("/v1/products", get(list_products).post(create_product))
        .route("/v1/categories", get(list_categories).post(create_category))
        .route(
            "/v1/categories/reorder",
            post(reorder_categories),
        )
        .route(
            "/v1/categories/{id}",
            get(get_category)
                .patch(update_category)
                .delete(delete_category),
        )
        .route("/v1/categories/{id}/image", put(update_category_image))
        .route("/v1/products/{id}", get(get_product).patch(update_product))
        .route(
            "/v1/products/{id}/images",
            get(list_product_images).post(attach_product_image),
        )
        .route(
            "/v1/products/{id}/images/{imageId}",
            axum::routing::delete(delete_product_image),
        )
        .route(
            "/v1/inventory/products/{productId}/balance",
            get(get_stock_balance),
        )
        .route("/v1/inventory/balances", get(list_stock_balances))
        .route("/v1/inventory/movements", post(record_movement))
        .route(
            "/v1/inventory/products/{productId}/movements",
            get(list_movements),
        )
        .route("/v1/sales", post(create_sale).get(list_sales))
        .route("/v1/sales/{id}", get(get_sale))
        .route("/v1/sales/{id}/confirm", post(confirm_sale))
        .route("/v1/sales/{id}/cancel", post(cancel_sale))
        .route("/v1/sales/{id}/declare-payment", post(declare_sale_payment))
        .route("/v1/portal/products", get(list_portal_products))
        .route("/v1/portal/categories", get(list_portal_categories))
        .route(
            "/v1/portal/categories/{slug}",
            get(get_portal_category_by_slug),
        )
        .route(
            "/v1/portal/orders",
            get(list_portal_orders).post(create_portal_order),
        )
        .route(
            "/v1/portal/orders/{id}",
            get(get_portal_order)
                .put(update_portal_order)
                .delete(cancel_portal_order),
        )
        .route("/v1/portal/orders/{id}/submit", post(submit_portal_order))
        .route("/v1/orders", get(list_orders))
        .route("/v1/orders/{id}", get(get_order))
        .route("/v1/orders/{id}/approve", post(approve_order))
        .route("/v1/orders/{id}/reject", post(reject_order_handler))
        .route("/v1/orders/{id}/cancel", post(cancel_order))
        .route("/v1/orders/{id}/start-picking", post(start_picking))
        .route("/v1/orders/{id}/delivery", post(create_order_delivery))
        .route("/v1/deliveries", get(list_deliveries))
        .route("/v1/deliveries/{id}", get(get_delivery))
        .route(
            "/v1/deliveries/{id}/start-transit",
            post(start_delivery_transit),
        )
        .route("/v1/deliveries/{id}/confirm", post(confirm_delivery))
        .route("/v1/media/upload", post(upload_media))
        .route("/v1/media/{id}/url", get(get_media_url))
        .route("/v1/media/{id}/content", get(get_media_content))
        .route("/v1/settings", get(get_settings).patch(patch_settings))
        .route("/v1/settings/logo", put(update_site_logo))
        .route("/v1/reports", post(generate_report).get(list_reports))
        .route("/v1/reports/{id}", get(get_report))
        .route("/v1/reports/{id}/export", get(export_report))
        .route("/v1/audit/events", get(list_audit_events))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state);

    public.merge(protected)
}

pub fn router() -> Router {
    health_router()
}

pub fn app_with_state(state: AppState) -> Router {
    health_router().merge(v1_router(state))
}
