use axum::{
    Json, Router,
    routing::{delete, get, patch, post, put},
};
use serde::Serialize;

use crate::admin_orders::{
    approve_order, cancel_order, get_order, list_orders, reject_order_handler, start_picking,
};
use crate::audit::list_audit_events;
use crate::billing::asaas_webhook;
use crate::billing::{attach_payment_method, get_invoice, get_subscription, list_invoices};
use crate::health::readiness;
use crate::maintenance::maintenance_middleware;
use crate::platform::{
    add_blocklist_entry, create_tenant, delete_blocklist_entry, disable_platform_user,
    enable_platform_user, end_impersonation, get_platform_user, get_tenant, get_tenant_stats,
    health_history, health_matrix, list_fraud_events, list_platform_tenants, list_platform_users, list_tenant_orders_support,
    list_tenant_products_support, list_tenant_sales_support, list_tenant_workforce,
    offboard_tenant, patch_platform_user, patch_tenant, patch_tenant_features,
    platform_login, platform_logout, platform_mfa_verify, platform_refresh, reactivate_tenant,
    reset_platform_user_password, resolve_fraud_event, run_dunning_job, run_offboarding_job,
    schedule_maintenance, start_impersonation, suspend_tenant,
};
use crate::platform::auth::platform_auth_middleware;
use crate::auth::{auth_middleware, login, logout, refresh};
use crate::tenant_gate::tenant_gate_middleware;
use crate::catalog_events::stream_catalog_events;
use crate::categories::{
    create_category, delete_category, get_category, list_categories, reorder_categories,
    update_category, update_category_image,
};
use crate::commerces::{
    approve_registration, create_address, create_commerce, deactivate_commerce, get_commerce,
    get_registration, list_addresses, list_commerces, list_registrations, lookup_cnpj,
    patch_registration, reject_registration, submit_registration, update_address, update_logo,
};
use crate::deliveries::{
    confirm_delivery, create_order_delivery, get_delivery, list_deliveries, start_delivery_transit,
};
use crate::domains::{
    create_domain, delete_domain, force_verify_platform_domain, get_domain_verify, host_tenant_middleware,
    list_domains, list_platform_domains, patch_platform_domain, run_domain_verification_job_handler,
    set_primary_domain,
};
use crate::fraud::list_fraud_alerts;
use crate::inventory::{get_stock_balance, list_movements, list_stock_balances, record_movement};
use crate::media::{
    get_media_content, get_media_url, get_public_product_media_content, upload_media,
};
use crate::portal::{
    cancel_portal_order, create_portal_order, get_portal_category_by_slug, get_portal_order,
    get_portal_product_by_id, get_public_category_by_slug, get_public_product_by_id,
    list_portal_categories, list_portal_orders, list_portal_products, list_public_banners,
    list_public_categories, list_public_featured_products, list_public_popular_products,
    list_public_products, list_public_promotions, submit_portal_order, update_portal_order,
};
use crate::portal_content::{
    create_admin_banner, create_admin_promotion, delete_admin_banner, delete_admin_promotion,
    list_admin_banners, list_admin_promotions, update_admin_banner, update_admin_promotion,
};
use crate::products::{
    attach_product_image, create_product, delete_product_image, get_product, list_product_images,
    list_products, list_top_selling_products, update_product,
};
use crate::reports::{export_report, generate_report, get_report, list_reports, verify_report};
use crate::sales::{
    cancel_sale, confirm_sale, create_sale, declare_sale_payment, get_sale, list_sales,
};
use crate::settings::{
    connect_asaas, disconnect_asaas, get_payment_balance, get_payment_settings,
    get_settings, get_public_settings, list_payment_transactions, patch_settings,
    update_payment_settings, update_site_logo,
};
use crate::status::public_status;
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

pub fn health_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/health/ready", get(readiness))
        .route("/v1/", get(v1_root))
        .with_state(state)
}

pub fn health_router_liveness_only() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/", get(v1_root))
}

pub fn v1_router(state: AppState) -> Router {
    let public = Router::new()
        .route("/v1/auth/login", post(login))
        .route("/v1/auth/refresh", post(refresh))
        .route("/v1/public/products", get(list_public_products))
        .route("/v1/public/products/featured", get(list_public_featured_products))
        .route("/v1/public/products/popular", get(list_public_popular_products))
        .route("/v1/public/products/{id}", get(get_public_product_by_id))
        .route("/v1/public/banners", get(list_public_banners))
        .route("/v1/public/promotions", get(list_public_promotions))
        .route("/v1/public/categories", get(list_public_categories))
        .route(
            "/v1/public/categories/{slug}",
            get(get_public_category_by_slug),
        )
        .route(
            "/v1/public/media/{id}/content",
            get(get_public_product_media_content),
        )
        .route("/v1/public/catalog/events", get(stream_catalog_events))
        .route("/v1/public/settings", get(get_public_settings))
        .route("/v1/status", get(public_status))
        .route("/v1/reports/{id}/verify", get(verify_report))
        .route("/v1/billing/webhooks/asaas", post(asaas_webhook))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            maintenance_middleware,
        ))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            host_tenant_middleware,
        ))
        .with_state(state.clone());

    let protected = Router::new()
        .route("/v1/auth/logout", post(logout))
        .route("/v1/users", post(create_user).get(list_users))
        .route("/v1/users/{id}", get(get_user))
        .route("/v1/users/{id}/deactivate", patch(deactivate_user))
        .route("/v1/users/{id}/driver-profile", put(upsert_driver_profile))
        .route("/v1/users/{id}/seller-profile", put(upsert_seller_profile))
        .route("/v1/commerces", post(create_commerce).get(list_commerces))
        .route("/v1/commerces/cnpj-lookup", get(lookup_cnpj))
        .route(
            "/v1/commerces/registrations",
            post(submit_registration).get(list_registrations),
        )
        .route(
            "/v1/commerces/registrations/{id}",
            get(get_registration).patch(patch_registration),
        )
        .route(
            "/v1/commerces/registrations/{id}/approve",
            post(approve_registration),
        )
        .route(
            "/v1/commerces/registrations/{id}/reject",
            post(reject_registration),
        )
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
        .route("/v1/products/top-selling", get(list_top_selling_products))
        .route("/v1/categories", get(list_categories).post(create_category))
        .route("/v1/categories/reorder", post(reorder_categories))
        .route(
            "/v1/categories/{id}",
            get(get_category)
                .patch(update_category)
                .delete(delete_category),
        )
        .route("/v1/categories/{id}/image", put(update_category_image))
        .route("/v1/portal/banners", get(list_admin_banners).post(create_admin_banner))
        .route(
            "/v1/portal/banners/{id}",
            patch(update_admin_banner).delete(delete_admin_banner),
        )
        .route(
            "/v1/portal/promotions",
            get(list_admin_promotions).post(create_admin_promotion),
        )
        .route(
            "/v1/portal/promotions/{id}",
            patch(update_admin_promotion).delete(delete_admin_promotion),
        )
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
        .route("/v1/portal/products/{id}", get(get_portal_product_by_id))
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
        .route("/v1/settings/domains", get(list_domains).post(create_domain))
        .route(
            "/v1/settings/domains/{id}/verify",
            get(get_domain_verify),
        )
        .route(
            "/v1/settings/domains/{id}/set-primary",
            post(set_primary_domain),
        )
        .route("/v1/settings/domains/{id}", delete(delete_domain))
        .route(
            "/v1/settings/payments",
            get(get_payment_settings).put(update_payment_settings),
        )
        .route(
            "/v1/settings/payments/asaas/connect",
            post(connect_asaas).delete(disconnect_asaas),
        )
        .route("/v1/settings/payments/balance", get(get_payment_balance))
        .route(
            "/v1/settings/payments/transactions",
            get(list_payment_transactions),
        )
        .route("/v1/reports", post(generate_report).get(list_reports))
        .route("/v1/reports/{id}", get(get_report))
        .route("/v1/reports/{id}/export", get(export_report))
        .route("/v1/audit/events", get(list_audit_events))
        .route("/v1/fraud/alerts", get(list_fraud_alerts))
        .route("/v1/billing/subscription", get(get_subscription))
        .route("/v1/billing/invoices", get(list_invoices))
        .route("/v1/billing/invoices/{id}", get(get_invoice))
        .route("/v1/billing/payment-methods", post(attach_payment_method))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            maintenance_middleware,
        ))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            tenant_gate_middleware,
        ))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state.clone());

    let platform_public = Router::new()
        .route("/v1/platform/auth/login", post(platform_login))
        .route("/v1/platform/auth/mfa/verify", post(platform_mfa_verify))
        .route("/v1/platform/auth/refresh", post(platform_refresh))
        .with_state(state.clone());

    let platform_protected = Router::new()
        .route("/v1/platform/auth/logout", post(platform_logout))
        .route("/v1/platform/users", get(list_platform_users))
        .route("/v1/platform/users/{id}", get(get_platform_user).patch(patch_platform_user))
        .route("/v1/platform/users/{id}/disable", post(disable_platform_user))
        .route("/v1/platform/users/{id}/enable", post(enable_platform_user))
        .route(
            "/v1/platform/users/{id}/reset-password",
            post(reset_platform_user_password),
        )
        .route(
            "/v1/platform/tenants",
            get(list_platform_tenants).post(create_tenant),
        )
        .route("/v1/platform/tenants/{id}", get(get_tenant).patch(patch_tenant))
        .route("/v1/platform/tenants/{id}/users", get(list_tenant_workforce))
        .route("/v1/platform/tenants/{id}/stats", get(get_tenant_stats))
        .route("/v1/platform/tenants/{id}/features", patch(patch_tenant_features))
        .route("/v1/platform/tenants/{id}/orders", get(list_tenant_orders_support))
        .route("/v1/platform/tenants/{id}/sales", get(list_tenant_sales_support))
        .route("/v1/platform/tenants/{id}/products", get(list_tenant_products_support))
        .route("/v1/platform/maintenance", post(schedule_maintenance))
        .route("/v1/platform/health/matrix", get(health_matrix))
        .route("/v1/platform/health/history", get(health_history))
        .route("/v1/platform/tenants/{id}/suspend", post(suspend_tenant))
        .route("/v1/platform/tenants/{id}/reactivate", post(reactivate_tenant))
        .route("/v1/platform/tenants/{id}/offboard", post(offboard_tenant))
        .route("/v1/platform/jobs/offboarding", post(run_offboarding_job))
        .route("/v1/platform/jobs/dunning", post(run_dunning_job))
        .route(
            "/v1/platform/jobs/domain-verification",
            post(run_domain_verification_job_handler),
        )
        .route("/v1/platform/impersonate", post(start_impersonation))
        .route("/v1/platform/impersonate/end", post(end_impersonation))
        .route("/v1/platform/fraud/events", get(list_fraud_events))
        .route(
            "/v1/platform/fraud/events/{id}/resolve",
            post(resolve_fraud_event),
        )
        .route("/v1/platform/domains", get(list_platform_domains))
        .route(
            "/v1/platform/domains/{id}",
            patch(patch_platform_domain),
        )
        .route(
            "/v1/platform/domains/{id}/force-verify",
            post(force_verify_platform_domain),
        )
        .route("/v1/platform/blocklist", post(add_blocklist_entry))
        .route("/v1/platform/blocklist/{id}", delete(delete_blocklist_entry))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            platform_auth_middleware,
        ))
        .with_state(state);

    public
        .merge(protected)
        .merge(platform_public)
        .merge(platform_protected)
}

pub fn router() -> Router {
    health_router_liveness_only()
}

pub fn app_with_state(state: AppState) -> Router {
    health_router(state.clone()).merge(v1_router(state))
}
