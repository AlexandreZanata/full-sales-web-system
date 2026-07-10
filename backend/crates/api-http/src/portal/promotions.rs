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
pub struct PublicPromotionsQuery {
    #[serde(default = "default_promotion_limit")]
    pub limit: u32,
}

fn default_promotion_limit() -> u32 {
    4
}

#[derive(Serialize)]
pub struct PortalPromotionResponse {
    pub id: uuid::Uuid,
    pub headline: String,
    #[serde(rename = "discountText")]
    pub discount_text: String,
    pub background: String,
    #[serde(rename = "categorySlug", skip_serializing_if = "Option::is_none")]
    pub category_slug: Option<String>,
    #[serde(rename = "linkUrl", skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

pub async fn list_public_promotions(
    State(state): State<AppState>,
    PublicTenantId(tenant_id): PublicTenantId,
    Query(query): Query<PublicPromotionsQuery>,
) -> Result<Json<CursorListResponse<PortalPromotionResponse>>, Response> {
    let limit = query.limit.clamp(1, 20) as i64;
    let rows = infra_postgres::portal::promotions::list_active_promotions(
        &state.app_pool,
        tenant_id,
        limit,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let ttl = Duration::from_secs(DEFAULT_PRESIGN_TTL_SECS);
    let mut data = Vec::with_capacity(rows.len());
    for row in rows {
        let image_url = match (&row.bucket, &row.object_key) {
            (Some(bucket), Some(key)) => state
                .storage
                .presigned_get(bucket, key, ttl)
                .await
                .ok()
                .map(|presigned| presigned.url),
            _ => None,
        };
        data.push(PortalPromotionResponse {
            id: row.id,
            headline: row.headline,
            discount_text: row.discount_text,
            background: row.background,
            category_slug: row.category_slug,
            link_url: row.link_url,
            image_url,
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
