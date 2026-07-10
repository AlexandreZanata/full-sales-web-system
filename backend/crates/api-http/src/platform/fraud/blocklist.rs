use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use domain_fraud::BlocklistEntry;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiError;
use crate::platform::auth::PlatformAuthUser;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateBlocklistRequest {
    pub email: Option<String>,
    pub cnpj: Option<String>,
    pub ip: Option<String>,
    #[serde(rename = "cardFingerprint")]
    pub card_fingerprint: Option<String>,
    pub reason: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct BlocklistEntryResponse {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cnpj: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "cardFingerprint", skip_serializing_if = "Option::is_none")]
    pub card_fingerprint: Option<String>,
    pub reason: String,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

pub async fn add_blocklist_entry(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    Json(body): Json<CreateBlocklistRequest>,
) -> Result<(StatusCode, Json<BlocklistEntryResponse>), ApiError> {
    let id = Uuid::now_v7();
    let entry = BlocklistEntry::new(
        id,
        body.email,
        body.cnpj,
        body.ip,
        body.card_fingerprint,
        body.reason,
        body.expires_at,
        auth.user_id,
    )
    .map_err(|_| ApiError::bad_request("INVALID_INPUT", "Invalid blocklist entry"))?;
    infra_postgres::fraud::insert_blocklist_entry(
        &state.admin_pool,
        infra_postgres::fraud::NewBlocklistEntry {
            id: entry.id,
            email: entry.email.clone(),
            cnpj: entry.cnpj.clone(),
            ip: entry.ip.clone(),
            card_fingerprint: entry.card_fingerprint.clone(),
            reason: entry.reason.clone(),
            expires_at: entry.expires_at,
            created_by: entry.created_by,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    Ok((
        StatusCode::CREATED,
        Json(BlocklistEntryResponse {
            id: entry.id,
            email: entry.email,
            cnpj: entry.cnpj,
            ip: entry.ip,
            card_fingerprint: entry.card_fingerprint,
            reason: entry.reason,
            expires_at: entry.expires_at,
            created_at: entry.created_at,
        }),
    ))
}

pub async fn delete_blocklist_entry(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let deleted = infra_postgres::fraud::delete_blocklist_entry(&state.admin_pool, id)
        .await
        .map_err(|_| ApiError::internal())?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::not_found())
    }
}
