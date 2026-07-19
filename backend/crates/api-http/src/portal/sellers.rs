//! Public seller share-link resolve (Phase 19).

use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;

use crate::domains::PublicTenantId;
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicSellerResponse {
    pub public_code: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
}

pub async fn get_public_seller_by_code(
    State(state): State<AppState>,
    PublicTenantId(tenant_id): PublicTenantId,
    Path(public_code): Path<String>,
) -> Result<Json<PublicSellerResponse>, ApiError> {
    let code = public_code.trim();
    if code.len() < 3 || code.len() > 32 {
        return Err(ApiError::not_found());
    }

    let row =
        infra_postgres::identity::find_public_seller_by_code(&state.app_pool, tenant_id, code)
            .await
            .map_err(|_| ApiError::internal())?
            .ok_or_else(ApiError::not_found)?;

    Ok(Json(PublicSellerResponse {
        public_code: row.public_code,
        display_name: row.display_name,
        contact_phone: row.contact_phone,
    }))
}
