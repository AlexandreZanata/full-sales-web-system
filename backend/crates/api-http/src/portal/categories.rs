use axum::{
    Json,
    extract::{Path, RawQuery, State},
    response::{IntoResponse, Response},
};
use domain_identity::Role;
use domain_shared::TenantId;
use serde::Serialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::list_query::{
    CursorListResponse, PORTAL_CATEGORIES_LIST_CONFIG, build_cursor_page, decode_query_pairs,
    parse_list_query,
};
use crate::portal::products::{
    PortalProductResponse, build_portal_product_responses, resolve_public_catalog_tenant,
};
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
    pub pagination: crate::list_query::CursorPaginationMeta,
}

pub async fn list_public_categories(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<CategoryResponse>>, Response> {
    let tenant_id = resolve_public_catalog_tenant().map_err(IntoResponse::into_response)?;
    list_categories_cursor(&state, tenant_id, query.as_deref())
        .await
        .map(Json)
        .map_err(IntoResponse::into_response)
}

pub async fn get_public_category_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    RawQuery(query): RawQuery,
) -> Result<Json<CategoryWithProductsResponse>, Response> {
    let tenant_id = resolve_public_catalog_tenant().map_err(IntoResponse::into_response)?;
    category_with_products(&state, tenant_id, &slug, query.as_deref())
        .await
        .map(Json)
        .map_err(IntoResponse::into_response)
}

pub async fn list_portal_categories(
    State(state): State<AppState>,
    auth: AuthUser,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<CategoryResponse>>, Response> {
    require_commerce_contact(&auth).map_err(IntoResponse::into_response)?;
    list_categories_cursor(&state, auth.tenant_id, query.as_deref())
        .await
        .map(Json)
        .map_err(IntoResponse::into_response)
}

pub async fn get_portal_category_by_slug(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(slug): Path<String>,
    RawQuery(query): RawQuery,
) -> Result<Json<CategoryWithProductsResponse>, Response> {
    require_commerce_contact(&auth).map_err(IntoResponse::into_response)?;
    category_with_products(&state, auth.tenant_id, &slug, query.as_deref())
        .await
        .map(Json)
        .map_err(IntoResponse::into_response)
}

async fn list_categories_cursor(
    state: &AppState,
    tenant_id: TenantId,
    query: Option<&str>,
) -> Result<CursorListResponse<CategoryResponse>, ApiError> {
    let parsed = parse_list_query(&decode_query_pairs(query), &PORTAL_CATEGORIES_LIST_CONFIG)
        .map_err(|err| ApiError::bad_request(err.code, err.message))?;
    let rows = infra_postgres::inventory::product_categories::list_categories_cursor(
        &state.app_pool,
        tenant_id,
        Some(true),
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        items.push(category_response(state, tenant_id, &row, false).await?);
    }
    Ok(build_cursor_page(
        items,
        parsed.pagination.limit,
        |category| category.id,
    ))
}

async fn category_with_products(
    state: &AppState,
    tenant_id: TenantId,
    slug: &str,
    query: Option<&str>,
) -> Result<CategoryWithProductsResponse, ApiError> {
    let row = infra_postgres::inventory::product_categories::find_category_by_slug(
        &state.app_pool,
        tenant_id,
        slug,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .filter(|category| category.active)
    .ok_or_else(|| ApiError::not_found_with_code("CATEGORY_NOT_FOUND", "Category not found"))?;

    let parsed = parse_list_query(&decode_query_pairs(query), &PORTAL_CATEGORIES_LIST_CONFIG)
        .map_err(|err| ApiError::bad_request(err.code, err.message))?;

    let product_rows = infra_postgres::inventory::list_portal_products_cursor(
        &state.app_pool,
        tenant_id,
        Some(slug),
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let products = build_portal_product_responses(state, tenant_id, &product_rows).await?;
    let page = build_cursor_page(products, parsed.pagination.limit, |product| product.id);

    Ok(CategoryWithProductsResponse {
        category: category_response(state, tenant_id, &row, true).await?,
        products: page.data,
        pagination: page.pagination,
    })
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

/// Missing blob must not fail category list/detail/PUT image.
pub(crate) async fn category_thumb_url(
    state: &AppState,
    tenant_id: TenantId,
    image_file_id: Option<Uuid>,
) -> Result<Option<String>, ApiError> {
    let Some(file_id) = image_file_id else {
        return Ok(None);
    };
    let Some(file) = infra_postgres::media::find_file_by_id(&state.app_pool, tenant_id, file_id)
        .await
        .map_err(|_| ApiError::internal())?
    else {
        return Ok(None);
    };
    let ttl =
        std::time::Duration::from_secs(infra_storage::object_storage::DEFAULT_PRESIGN_TTL_SECS);
    match state
        .storage
        .presigned_get(&file.bucket, &file.object_key, ttl)
        .await
    {
        Ok(presigned) => Ok(Some(crate::media::catalog_image_url(
            file_id,
            &presigned.url,
        ))),
        Err(_) => Ok(None),
    }
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
