use std::collections::HashMap;
use std::time::Duration;

use axum::{
    Json,
    extract::{RawQuery, State},
    response::{IntoResponse, Response},
};
use domain_identity::Role;
use domain_shared::TenantId;
use infra_storage::object_storage::DEFAULT_PRESIGN_TTL_SECS;
use serde::Serialize;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::list_query::{
    PORTAL_PRODUCTS_LIST_CONFIG, CursorListResponse, build_cursor_page, decode_query_pairs,
    filter_eq_string, parse_list_query,
};
use crate::state::AppState;

const DEV_SEED_TENANT_ID: &str = "01900001-0000-7000-8000-000000000001";

#[derive(Serialize)]
pub struct PortalProductResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub sku: String,
    #[serde(rename = "priceAmount")]
    pub price_amount: i64,
    #[serde(rename = "priceCurrency")]
    pub price_currency: String,
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<uuid::Uuid>,
    #[serde(rename = "categoryName", skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[serde(rename = "categorySlug", skip_serializing_if = "Option::is_none")]
    pub category_slug: Option<String>,
    #[serde(rename = "primaryImageUrl")]
    pub primary_image_url: Option<String>,
}

pub async fn list_portal_products(
    State(state): State<AppState>,
    auth: AuthUser,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<PortalProductResponse>>, Response> {
    let _commerce_id = require_commerce_contact(&auth).map_err(IntoResponse::into_response)?;
    list_products_cursor(&state, auth.tenant_id, query.as_deref())
        .await
        .map(Json)
        .map_err(IntoResponse::into_response)
}

pub async fn list_public_products(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<PortalProductResponse>>, Response> {
    let tenant_id = resolve_public_catalog_tenant().map_err(IntoResponse::into_response)?;
    list_products_cursor(&state, tenant_id, query.as_deref())
        .await
        .map(Json)
        .map_err(IntoResponse::into_response)
}

pub(crate) fn resolve_public_catalog_tenant() -> Result<TenantId, ApiError> {
    if let Ok(raw) = std::env::var("PUBLIC_CATALOG_TENANT_ID") {
        return TenantId::parse(raw.trim()).map_err(|_| ApiError::internal());
    }
    TenantId::parse(DEV_SEED_TENANT_ID).map_err(|_| ApiError::internal())
}

pub(crate) async fn list_products_cursor(
    state: &AppState,
    tenant_id: TenantId,
    query: Option<&str>,
) -> Result<CursorListResponse<PortalProductResponse>, ApiError> {
    let parsed = parse_list_query(
        &decode_query_pairs(query),
        &PORTAL_PRODUCTS_LIST_CONFIG,
    )
    .map_err(|err| ApiError::bad_request(&err.code, &err.message))?;
    let category_slug = filter_eq_string(&parsed.filters, "category_slug");
    let rows = infra_postgres::inventory::list_portal_products_cursor(
        &state.app_pool,
        tenant_id,
        category_slug.as_deref(),
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let items = build_portal_product_responses(state, tenant_id, &rows).await?;
    Ok(build_cursor_page(
        items,
        parsed.pagination.limit,
        |product| product.id,
    ))
}

pub(crate) async fn build_portal_product_responses(
    state: &AppState,
    tenant_id: TenantId,
    rows: &[infra_postgres::inventory::ProductRow],
) -> Result<Vec<PortalProductResponse>, ApiError> {
    let product_ids: Vec<uuid::Uuid> = rows.iter().map(|row| row.id).collect();
    let images = infra_postgres::inventory::product_images::find_primary_images_for_products(
        &state.app_pool,
        tenant_id,
        &product_ids,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let mut image_by_product: HashMap<uuid::Uuid, (uuid::Uuid, String, String)> = HashMap::new();
    for image in images {
        image_by_product.insert(
            image.product_id,
            (image.file_id, image.bucket, image.object_key),
        );
    }

    let ttl = Duration::from_secs(DEFAULT_PRESIGN_TTL_SECS);
    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        let primary_image_url = match image_by_product.get(&row.id) {
            Some((file_id, bucket, key)) => state
                .storage
                .presigned_get(bucket, key, ttl)
                .await
                .ok()
                .map(|presigned| crate::media::catalog_image_url(*file_id, &presigned.url)),
            None => None,
        };
        items.push(portal_product_from_row(row, primary_image_url));
    }
    Ok(items)
}

pub(crate) fn portal_product_from_row(
    row: &infra_postgres::inventory::ProductRow,
    primary_image_url: Option<String>,
) -> PortalProductResponse {
    PortalProductResponse {
        id: row.id,
        name: row.name.clone(),
        sku: row.sku.clone(),
        price_amount: row.price_amount,
        price_currency: row.price_currency.clone(),
        category_id: row.category_id,
        category_name: row.category_name.clone(),
        category_slug: row.category_slug.clone(),
        primary_image_url,
    }
}

pub(super) fn require_commerce_contact(auth: &AuthUser) -> Result<uuid::Uuid, ApiError> {
    if auth.role != Role::CommerceContact {
        return Err(ApiError::forbidden());
    }
    auth.commerce_id.ok_or_else(ApiError::forbidden)
}
