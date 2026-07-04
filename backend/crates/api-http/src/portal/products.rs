use std::collections::HashMap;
use std::time::Duration;

use axum::{Json, extract::{Query, State}};
use domain_identity::Role;
use infra_storage::object_storage::DEFAULT_PRESIGN_TTL_SECS;
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct PortalProductsQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(rename = "pageSize", default = "default_page_size")]
    pub page_size: u32,
    pub category: Option<String>,
}

pub(super) fn default_page() -> u32 {
    1
}

pub(super) fn default_page_size() -> u32 {
    20
}

#[derive(Serialize)]
pub struct PortalProductResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub sku: String,
    #[serde(rename = "priceAmount")]
    pub price_amount: i64,
    #[serde(rename = "priceCurrency")]
    pub price_currency: String,
    pub category: Option<String>,
    #[serde(rename = "primaryImageUrl")]
    pub primary_image_url: Option<String>,
}

#[derive(Serialize)]
pub struct PaginatedPortalProductsResponse {
    pub items: Vec<PortalProductResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
}

pub async fn list_portal_products(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<PortalProductsQuery>,
) -> Result<Json<PaginatedPortalProductsResponse>, ApiError> {
    let _commerce_id = require_commerce_contact(&auth)?;
    let page_size = query.page_size.clamp(1, 50);
    let page = query.page.max(1);
    let offset = ((page - 1) as i64) * (page_size as i64);
    let category = query.category.as_deref();

    let rows = infra_postgres::inventory::list_portal_products(
        &state.app_pool,
        auth.tenant_id,
        category,
        page_size as i64,
        offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let total = infra_postgres::inventory::count_portal_products(
        &state.app_pool,
        auth.tenant_id,
        category,
    )
    .await
    .map_err(|_| ApiError::internal())? as u64;

    let product_ids: Vec<uuid::Uuid> = rows.iter().map(|row| row.id).collect();
    let images = infra_postgres::inventory::product_images::find_primary_images_for_products(
        &state.app_pool,
        auth.tenant_id,
        &product_ids,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let mut image_by_product: HashMap<uuid::Uuid, (String, String)> = HashMap::new();
    for image in images {
        image_by_product.insert(image.product_id, (image.bucket, image.object_key));
    }

    let ttl = Duration::from_secs(DEFAULT_PRESIGN_TTL_SECS);
    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        let primary_image_url = match image_by_product.get(&row.id) {
            Some((bucket, key)) => state
                .storage
                .presigned_get(bucket, key, ttl)
                .await
                .ok()
                .map(|p| p.url),
            None => None,
        };
        items.push(PortalProductResponse {
            id: row.id,
            name: row.name,
            sku: row.sku,
            price_amount: row.price_amount,
            price_currency: row.price_currency,
            category: row.category,
            primary_image_url,
        });
    }

    Ok(Json(PaginatedPortalProductsResponse {
        items,
        page,
        page_size,
        total,
    }))
}

pub(super) fn require_commerce_contact(auth: &AuthUser) -> Result<uuid::Uuid, ApiError> {
    if auth.role != Role::CommerceContact {
        return Err(ApiError::forbidden());
    }
    auth.commerce_id.ok_or_else(ApiError::forbidden)
}
