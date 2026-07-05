use axum::{
    Json,
    extract::{Path, Query, State},
};
use domain_identity::Role;
use domain_shared::TenantId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::portal::products::{PortalProductResponse, list_products_for_tenant, resolve_public_catalog_tenant, PortalProductsQuery};
use crate::state::AppState;

#[derive(Serialize)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: i32,
    pub active: bool,
    #[serde(rename = "imageFileId", skip_serializing_if = "Option::is_none")]
    pub image_file_id: Option<Uuid>,
    #[serde(rename = "thumbUrl", skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(rename = "productCount", skip_serializing_if = "Option::is_none")]
    pub product_count: Option<i64>,
}

#[derive(Serialize)]
pub struct CategoryWithProductsResponse {
    #[serde(flatten)]
    pub category: CategoryResponse,
    pub products: Vec<PortalProductResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
}

#[derive(Deserialize)]
pub struct CategorySlugQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(rename = "pageSize", default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

pub async fn list_public_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<CategoryResponse>>, ApiError> {
    let tenant_id = resolve_public_catalog_tenant()?;
    list_active_category_responses(&state, tenant_id).await
}

pub async fn get_public_category_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<CategorySlugQuery>,
) -> Result<Json<CategoryWithProductsResponse>, ApiError> {
    let tenant_id = resolve_public_catalog_tenant()?;
    category_with_products(&state, tenant_id, &slug, &query).await
}

pub async fn list_portal_categories(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<CategoryResponse>>, ApiError> {
    require_commerce_contact(&auth)?;
    list_active_category_responses(&state, auth.tenant_id).await
}

pub async fn get_portal_category_by_slug(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(slug): Path<String>,
    Query(query): Query<CategorySlugQuery>,
) -> Result<Json<CategoryWithProductsResponse>, ApiError> {
    require_commerce_contact(&auth)?;
    category_with_products(&state, auth.tenant_id, &slug, &query).await
}

async fn list_active_category_responses(
    state: &AppState,
    tenant_id: TenantId,
) -> Result<Json<Vec<CategoryResponse>>, ApiError> {
    let rows = infra_postgres::inventory::product_categories::list_active_categories(
        &state.app_pool,
        tenant_id,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        items.push(category_response(state, tenant_id, &row, false).await?);
    }
    Ok(Json(items))
}

async fn category_with_products(
    state: &AppState,
    tenant_id: TenantId,
    slug: &str,
    query: &CategorySlugQuery,
) -> Result<Json<CategoryWithProductsResponse>, ApiError> {
    let row = infra_postgres::inventory::product_categories::find_category_by_slug(
        &state.app_pool,
        tenant_id,
        slug,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .filter(|category| category.active)
    .ok_or_else(|| ApiError::not_found_with_code("CATEGORY_NOT_FOUND", "Category not found"))?;

    let products_query = PortalProductsQuery {
        page: query.page,
        page_size: query.page_size,
        category: Some(slug.to_owned()),
    };
    let products = list_products_for_tenant(state, tenant_id, &products_query).await?;

    Ok(Json(CategoryWithProductsResponse {
        category: category_response(state, tenant_id, &row, true).await?,
        products: products.items,
        page: products.page,
        page_size: products.page_size,
        total: products.total,
    }))
}

pub(crate) async fn category_response(
    state: &AppState,
    tenant_id: TenantId,
    row: &infra_postgres::inventory::product_categories::CategoryRow,
    include_product_count: bool,
) -> Result<CategoryResponse, ApiError> {
    let thumb_url = category_thumb_url(state, tenant_id, row.image_file_id).await?;
    let product_count = if include_product_count {
        Some(
            infra_postgres::inventory::product_categories::count_products_in_category(
                &state.app_pool,
                tenant_id,
                row.id,
            )
            .await
            .map_err(|_| ApiError::internal())?,
        )
    } else {
        None
    };

    Ok(CategoryResponse {
        id: row.id,
        name: row.name.clone(),
        slug: row.slug.clone(),
        description: row.description.clone(),
        sort_order: row.sort_order,
        active: row.active,
        image_file_id: row.image_file_id,
        thumb_url,
        product_count,
    })
}

pub(crate) async fn category_thumb_url(
    state: &AppState,
    tenant_id: TenantId,
    image_file_id: Option<Uuid>,
) -> Result<Option<String>, ApiError> {
    let Some(file_id) = image_file_id else {
        return Ok(None);
    };
    let file = infra_postgres::media::find_file_by_id(&state.app_pool, tenant_id, file_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::internal)?;
    let ttl = std::time::Duration::from_secs(infra_storage::object_storage::DEFAULT_PRESIGN_TTL_SECS);
    let presigned = state
        .storage
        .presigned_get(&file.bucket, &file.object_key, ttl)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(Some(crate::media::catalog_image_url(
        file_id,
        &presigned.url,
    )))
}

fn require_commerce_contact(auth: &AuthUser) -> Result<(), ApiError> {
    if auth.role != Role::CommerceContact {
        return Err(ApiError::forbidden());
    }
    if auth.commerce_id.is_none() {
        return Err(ApiError::forbidden());
    }
    Ok(())
}
