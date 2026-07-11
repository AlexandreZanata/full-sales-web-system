use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::list_query::{CursorListResponse, CursorPaginationMeta};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct AdminListQuery {
    #[serde(default = "default_admin_limit")]
    pub limit: u32,
}

fn default_admin_limit() -> u32 {
    50
}

#[derive(Serialize)]
pub struct AdminBannerResponse {
    pub id: Uuid,
    pub placement: String,
    #[serde(rename = "imageFileId")]
    pub image_file_id: Uuid,
    #[serde(rename = "linkUrl", skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(rename = "altText", skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: i32,
    pub active: bool,
}

#[derive(Deserialize)]
pub struct CreateBannerRequest {
    #[serde(default = "default_placement")]
    pub placement: String,
    #[serde(rename = "imageFileId")]
    pub image_file_id: Uuid,
    #[serde(rename = "linkUrl")]
    pub link_url: Option<String>,
    #[serde(rename = "altText")]
    pub alt_text: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
}

fn default_placement() -> String {
    "hero".into()
}

#[derive(Deserialize, Default)]
pub struct UpdateBannerRequest {
    pub placement: Option<String>,
    #[serde(rename = "imageFileId")]
    pub image_file_id: Option<Uuid>,
    #[serde(rename = "linkUrl")]
    pub link_url: Option<Option<String>>,
    #[serde(rename = "altText")]
    pub alt_text: Option<Option<String>>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
}

fn banner_response(row: infra_postgres::portal::banners::BannerRow) -> AdminBannerResponse {
    AdminBannerResponse {
        id: row.id,
        placement: row.placement,
        image_file_id: row.image_file_id,
        link_url: row.link_url,
        alt_text: row.alt_text,
        sort_order: row.sort_order,
        active: row.active,
    }
}

pub async fn list_admin_banners(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<AdminListQuery>,
) -> Result<Json<CursorListResponse<AdminBannerResponse>>, ApiError> {
    require_admin(&auth)?;
    let rows = infra_postgres::portal::banners::list_banners(
        &state.app_pool,
        auth.tenant_id,
        query.limit.clamp(1, 100) as i64,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    Ok(Json(CursorListResponse {
        data: rows.into_iter().map(banner_response).collect(),
        pagination: CursorPaginationMeta {
            next_cursor: None,
            has_more: false,
            limit: query.limit,
        },
    }))
}

pub async fn create_admin_banner(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<CreateBannerRequest>,
) -> Result<(StatusCode, Json<AdminBannerResponse>), ApiError> {
    require_admin(&auth)?;
    let id = Uuid::now_v7();
    infra_postgres::portal::banners::insert_banner(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::portal::banners::BannerInsert {
            id,
            placement: body.placement,
            image_file_id: body.image_file_id,
            link_url: body.link_url,
            alt_text: body.alt_text,
            sort_order: body.sort_order.unwrap_or(0),
            active: body.active.unwrap_or(true),
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let row =
        infra_postgres::portal::banners::find_banner_by_id(&state.app_pool, auth.tenant_id, id)
            .await
            .map_err(|_| ApiError::internal())?
            .ok_or_else(ApiError::internal)?;
    Ok((StatusCode::CREATED, Json(banner_response(row))))
}

pub async fn update_admin_banner(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateBannerRequest>,
) -> Result<Json<AdminBannerResponse>, ApiError> {
    require_admin(&auth)?;
    let updated = infra_postgres::portal::banners::update_banner(
        &state.app_pool,
        auth.tenant_id,
        id,
        &infra_postgres::portal::banners::BannerUpdate {
            placement: body.placement,
            image_file_id: body.image_file_id,
            link_url: body.link_url,
            alt_text: body.alt_text,
            sort_order: body.sort_order,
            active: body.active,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::not_found_with_code(
            "BANNER_NOT_FOUND",
            "Banner not found",
        ));
    }
    let row =
        infra_postgres::portal::banners::find_banner_by_id(&state.app_pool, auth.tenant_id, id)
            .await
            .map_err(|_| ApiError::internal())?
            .ok_or_else(ApiError::internal)?;
    Ok(Json(banner_response(row)))
}

pub async fn delete_admin_banner(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    require_admin(&auth)?;
    let deleted =
        infra_postgres::portal::banners::delete_banner(&state.app_pool, auth.tenant_id, id)
            .await
            .map_err(|_| ApiError::internal())?;
    if !deleted {
        return Err(ApiError::not_found_with_code(
            "BANNER_NOT_FOUND",
            "Banner not found",
        ));
    }
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct AdminPromotionResponse {
    pub id: Uuid,
    pub headline: String,
    #[serde(rename = "discountText")]
    pub discount_text: String,
    pub background: String,
    #[serde(rename = "categorySlug", skip_serializing_if = "Option::is_none")]
    pub category_slug: Option<String>,
    #[serde(rename = "linkUrl", skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(rename = "imageFileId", skip_serializing_if = "Option::is_none")]
    pub image_file_id: Option<Uuid>,
    #[serde(rename = "sortOrder")]
    pub sort_order: i32,
    pub active: bool,
}

#[derive(Deserialize)]
pub struct CreatePromotionRequest {
    pub headline: String,
    #[serde(rename = "discountText")]
    pub discount_text: String,
    pub background: String,
    #[serde(rename = "categorySlug")]
    pub category_slug: Option<String>,
    #[serde(rename = "linkUrl")]
    pub link_url: Option<String>,
    #[serde(rename = "imageFileId")]
    pub image_file_id: Option<Uuid>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
}

#[derive(Deserialize, Default)]
pub struct UpdatePromotionRequest {
    pub headline: Option<String>,
    #[serde(rename = "discountText")]
    pub discount_text: Option<String>,
    pub background: Option<String>,
    #[serde(rename = "categorySlug")]
    pub category_slug: Option<Option<String>>,
    #[serde(rename = "linkUrl")]
    pub link_url: Option<Option<String>>,
    #[serde(rename = "imageFileId")]
    pub image_file_id: Option<Option<Uuid>>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
}

fn promotion_response(
    row: infra_postgres::portal::promotions::PromotionRow,
) -> AdminPromotionResponse {
    AdminPromotionResponse {
        id: row.id,
        headline: row.headline,
        discount_text: row.discount_text,
        background: row.background,
        category_slug: row.category_slug,
        link_url: row.link_url,
        image_file_id: row.image_file_id,
        sort_order: row.sort_order,
        active: row.active,
    }
}

pub async fn list_admin_promotions(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<AdminListQuery>,
) -> Result<Json<CursorListResponse<AdminPromotionResponse>>, ApiError> {
    require_admin(&auth)?;
    let rows = infra_postgres::portal::promotions::list_promotions(
        &state.app_pool,
        auth.tenant_id,
        query.limit.clamp(1, 100) as i64,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    Ok(Json(CursorListResponse {
        data: rows.into_iter().map(promotion_response).collect(),
        pagination: CursorPaginationMeta {
            next_cursor: None,
            has_more: false,
            limit: query.limit,
        },
    }))
}

pub async fn create_admin_promotion(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<CreatePromotionRequest>,
) -> Result<(StatusCode, Json<AdminPromotionResponse>), ApiError> {
    require_admin(&auth)?;
    validate_promotion_background(&body.background)?;
    let id = Uuid::now_v7();
    infra_postgres::portal::promotions::insert_promotion(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::portal::promotions::PromotionInsert {
            id,
            headline: body.headline,
            discount_text: body.discount_text,
            background: body.background,
            category_slug: body.category_slug,
            link_url: body.link_url,
            image_file_id: body.image_file_id,
            sort_order: body.sort_order.unwrap_or(0),
            active: body.active.unwrap_or(true),
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let row = infra_postgres::portal::promotions::find_promotion_by_id(
        &state.app_pool,
        auth.tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::internal)?;
    Ok((StatusCode::CREATED, Json(promotion_response(row))))
}

pub async fn update_admin_promotion(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePromotionRequest>,
) -> Result<Json<AdminPromotionResponse>, ApiError> {
    require_admin(&auth)?;
    if let Some(background) = body.background.as_deref() {
        validate_promotion_background(background)?;
    }
    let updated = infra_postgres::portal::promotions::update_promotion(
        &state.app_pool,
        auth.tenant_id,
        id,
        &infra_postgres::portal::promotions::PromotionUpdate {
            headline: body.headline,
            discount_text: body.discount_text,
            background: body.background,
            category_slug: body.category_slug,
            link_url: body.link_url,
            image_file_id: body.image_file_id,
            sort_order: body.sort_order,
            active: body.active,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::not_found_with_code(
            "PROMOTION_NOT_FOUND",
            "Promotion not found",
        ));
    }
    let row = infra_postgres::portal::promotions::find_promotion_by_id(
        &state.app_pool,
        auth.tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::internal)?;
    Ok(Json(promotion_response(row)))
}

pub async fn delete_admin_promotion(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    require_admin(&auth)?;
    let deleted =
        infra_postgres::portal::promotions::delete_promotion(&state.app_pool, auth.tenant_id, id)
            .await
            .map_err(|_| ApiError::internal())?;
    if !deleted {
        return Err(ApiError::not_found_with_code(
            "PROMOTION_NOT_FOUND",
            "Promotion not found",
        ));
    }
    Ok(StatusCode::NO_CONTENT)
}

fn validate_promotion_background(background: &str) -> Result<(), ApiError> {
    if background == "yellow" || background == "green" {
        return Ok(());
    }
    Err(ApiError::bad_request(
        "VALIDATION_ERROR",
        "background must be yellow or green",
    ))
}
