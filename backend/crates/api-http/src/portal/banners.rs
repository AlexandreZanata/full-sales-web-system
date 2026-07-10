use std::time::Duration;

use axum::{
    Json,
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use infra_storage::object_storage::DEFAULT_PRESIGN_TTL_SECS;
use serde::{Deserialize, Serialize};

use crate::domains::PublicTenantId;
use crate::error::ApiError;
use crate::list_query::{CursorListResponse, CursorPaginationMeta};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct PublicBannersQuery {
    #[serde(default = "default_placement")]
    pub placement: String,
    #[serde(default = "default_banner_limit")]
    pub limit: u32,
}

fn default_placement() -> String {
    "hero".into()
}

fn default_banner_limit() -> u32 {
    10
}

#[derive(Serialize)]
pub struct PortalBannerResponse {
    pub id: uuid::Uuid,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "linkUrl", skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(rename = "altText", skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
}

pub async fn list_public_banners(
    State(state): State<AppState>,
    PublicTenantId(tenant_id): PublicTenantId,
    Query(query): Query<PublicBannersQuery>,
) -> Result<Json<CursorListResponse<PortalBannerResponse>>, Response> {
    let limit = query.limit.clamp(1, 20) as i64;
    let rows = infra_postgres::portal::banners::list_active_banners_with_files(
        &state.app_pool,
        tenant_id,
        &query.placement,
        limit,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let ttl = Duration::from_secs(DEFAULT_PRESIGN_TTL_SECS);
    let mut data = Vec::with_capacity(rows.len());
    for row in rows {
        let presigned = state
            .storage
            .presigned_get(&row.bucket, &row.object_key, ttl)
            .await
            .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;
        data.push(PortalBannerResponse {
            id: row.id,
            image_url: crate::media::catalog_image_url(row.image_file_id, &presigned.url),
            link_url: row.link_url,
            alt_text: row.alt_text,
        });
    }

    Ok(Json(CursorListResponse {
        data,
        pagination: CursorPaginationMeta {
            next_cursor: None,
            has_more: false,
            limit: query.limit,
        },
    }))
}
