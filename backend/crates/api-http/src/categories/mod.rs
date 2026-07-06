use axum::{
    Json,
    extract::{Path, RawQuery, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain_shared::TenantId;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::catalog_events::notify_category_changed;
use crate::error::ApiError;
use crate::list_query::{
    CATEGORIES_LIST_CONFIG, CursorListResponse, build_cursor_page, decode_query_pairs,
    filter_eq_bool, parse_list_query,
};
use crate::portal::categories::{CategoryResponse, category_response};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
    pub slug: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
    pub slug: Option<String>,
}

#[derive(Deserialize)]
pub struct ReorderCategoriesRequest {
    #[serde(rename = "orderedIds")]
    pub ordered_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct UpdateCategoryImageRequest {
    #[serde(rename = "fileId")]
    pub file_id: Uuid,
}

pub async fn list_categories(
    State(state): State<AppState>,
    auth: AuthUser,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<CategoryResponse>>, Response> {
    require_admin(&auth).map_err(IntoResponse::into_response)?;
    let parsed = parse_list_query(
        &decode_query_pairs(query.as_deref()),
        &CATEGORIES_LIST_CONFIG,
    )
    .map_err(IntoResponse::into_response)?;
    let active = filter_eq_bool(&parsed.filters, "active");
    let rows = infra_postgres::inventory::product_categories::list_categories_cursor(
        &state.app_pool,
        auth.tenant_id,
        active,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        items.push(
            category_response(&state, auth.tenant_id, &row, true)
                .await
                .map_err(IntoResponse::into_response)?,
        );
    }

    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |category| category.id,
    )))
}

pub async fn create_category(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<CategoryResponse>), ApiError> {
    require_admin(&auth)?;
    if body.name.trim().is_empty() {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "Category name is required",
        ));
    }

    let existing = infra_postgres::inventory::product_categories::list_category_slugs(
        &state.app_pool,
        auth.tenant_id,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let id = Uuid::now_v7();
    let category = application::categories::create_category(
        id,
        auth.tenant_id,
        &body.name,
        body.slug.as_deref(),
        body.description.as_deref(),
        body.sort_order.unwrap_or(0),
        body.active.unwrap_or(true),
        existing,
    )
    .map_err(map_categories_error)?;

    infra_postgres::inventory::product_categories::insert_category(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::inventory::product_categories::CategoryInsert {
            id: category.id(),
            name: category.name().to_owned(),
            slug: category.slug().to_owned(),
            description: category.description().map(str::to_owned),
            sort_order: category.sort_order(),
            active: category.is_active(),
            image_file_id: None,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    notify_category_changed(&state.catalog_events, "created", category.id());

    let row = infra_postgres::inventory::product_categories::find_category_by_id(
        &state.app_pool,
        auth.tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::internal)?;

    Ok((
        StatusCode::CREATED,
        Json(category_response(&state, auth.tenant_id, &row, true).await?),
    ))
}

pub async fn get_category(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<CategoryResponse>, ApiError> {
    require_admin(&auth)?;
    let row = load_category(&state, auth.tenant_id, id).await?;
    category_response(&state, auth.tenant_id, &row, true)
        .await
        .map(Json)
}

pub async fn update_category(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateCategoryRequest>,
) -> Result<Json<CategoryResponse>, ApiError> {
    require_admin(&auth)?;
    let row = load_category(&state, auth.tenant_id, id).await?;

    let name = body
        .name
        .as_ref()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| row.name.clone());

    if body.name.is_some() && name.is_empty() {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "Category name is required",
        ));
    }

    let slug = if let Some(explicit) = body.slug.as_deref() {
        let slug = domain_inventory::slugify_name(explicit);
        if slug.is_empty() {
            return Err(ApiError::bad_request(
                "VALIDATION_ERROR",
                "Invalid category slug",
            ));
        }
        slug
    } else if body.name.is_some() {
        let existing = infra_postgres::inventory::product_categories::list_category_slugs(
            &state.app_pool,
            auth.tenant_id,
        )
        .await
        .map_err(|_| ApiError::internal())?
        .into_iter()
        .filter(|slug| slug != &row.slug)
        .collect::<Vec<_>>();
        let mut category = application::categories::create_category(
            row.id,
            auth.tenant_id,
            &name,
            None,
            row.description.as_deref(),
            row.sort_order,
            row.active,
            existing,
        )
        .map_err(map_categories_error)?;
        application::categories::rename_category(&mut category, &name, true, &[])
            .map_err(map_categories_error)?;
        category.slug().to_owned()
    } else {
        row.slug.clone()
    };

    let updated = infra_postgres::inventory::product_categories::update_category(
        &state.app_pool,
        auth.tenant_id,
        id,
        &infra_postgres::inventory::product_categories::CategoryUpdate {
            name: body.name.as_ref().map(|_| name),
            slug: if body.slug.is_some() || body.name.is_some() {
                Some(slug)
            } else {
                None
            },
            description: body.description.map(|value| {
                let trimmed = value.trim().to_owned();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            }),
            sort_order: body.sort_order,
            active: body.active,
            image_file_id: None,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::not_found_with_code(
            "CATEGORY_NOT_FOUND",
            "Category not found",
        ));
    }

    notify_category_changed(&state.catalog_events, "updated", id);
    let row = load_category(&state, auth.tenant_id, id).await?;
    category_response(&state, auth.tenant_id, &row, true)
        .await
        .map(Json)
}

pub async fn delete_category(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    require_admin(&auth)?;
    let _ = load_category(&state, auth.tenant_id, id).await?;
    let updated = infra_postgres::inventory::product_categories::update_category(
        &state.app_pool,
        auth.tenant_id,
        id,
        &infra_postgres::inventory::product_categories::CategoryUpdate {
            name: None,
            slug: None,
            description: None,
            sort_order: None,
            active: Some(false),
            image_file_id: None,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::not_found_with_code(
            "CATEGORY_NOT_FOUND",
            "Category not found",
        ));
    }
    notify_category_changed(&state.catalog_events, "deactivated", id);
    Ok(StatusCode::NO_CONTENT)
}

pub async fn reorder_categories(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<ReorderCategoriesRequest>,
) -> Result<StatusCode, ApiError> {
    require_admin(&auth)?;
    if body.ordered_ids.is_empty() {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "orderedIds must not be empty",
        ));
    }
    infra_postgres::inventory::product_categories::reorder_categories(
        &state.app_pool,
        auth.tenant_id,
        &body.ordered_ids,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    notify_category_changed(&state.catalog_events, "reordered", Uuid::nil());
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_category_image(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateCategoryImageRequest>,
) -> Result<Json<CategoryResponse>, ApiError> {
    require_admin(&auth)?;
    let _ = load_category(&state, auth.tenant_id, id).await?;
    let file =
        infra_postgres::media::find_file_by_id(&state.app_pool, auth.tenant_id, body.file_id)
            .await
            .map_err(|_| ApiError::internal())?
            .ok_or_else(ApiError::media_not_found)?;

    if file.entity_type != "ProductCategory" || file.entity_id != id {
        return Err(ApiError::media_not_found());
    }

    let updated = infra_postgres::inventory::product_categories::update_category(
        &state.app_pool,
        auth.tenant_id,
        id,
        &infra_postgres::inventory::product_categories::CategoryUpdate {
            name: None,
            slug: None,
            description: None,
            sort_order: None,
            active: None,
            image_file_id: Some(Some(body.file_id)),
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::not_found_with_code(
            "CATEGORY_NOT_FOUND",
            "Category not found",
        ));
    }

    notify_category_changed(&state.catalog_events, "updated", id);
    let row = load_category(&state, auth.tenant_id, id).await?;
    category_response(&state, auth.tenant_id, &row, true)
        .await
        .map(Json)
}

async fn load_category(
    state: &AppState,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<infra_postgres::inventory::product_categories::CategoryRow, ApiError> {
    infra_postgres::inventory::product_categories::find_category_by_id(
        &state.app_pool,
        tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(|| ApiError::not_found_with_code("CATEGORY_NOT_FOUND", "Category not found"))
}

fn map_categories_error(err: application::categories::CategoriesAppError) -> ApiError {
    match err {
        application::categories::CategoriesAppError::Inventory(
            domain_inventory::InventoryError::InvalidCategoryName
            | domain_inventory::InventoryError::InvalidCategorySlug,
        ) => ApiError::bad_request("VALIDATION_ERROR", "Invalid category data"),
        _ => ApiError::internal(),
    }
}
