//! Public + admin handlers for portal commerce interest leads.

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::domains::PublicTenantId;
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePortalLeadRequest {
    pub contact_name: String,
    pub phone: String,
    pub commerce_name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct ListPortalLeadsQuery {
    pub status: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewPortalLeadRequest {
    pub status: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalLeadResponse {
    pub id: Uuid,
    pub contact_name: String,
    pub phone: String,
    pub commerce_name: String,
    pub email: String,
    pub status: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
}

pub async fn create_public_portal_lead(
    State(state): State<AppState>,
    PublicTenantId(tenant_id): PublicTenantId,
    Json(body): Json<CreatePortalLeadRequest>,
) -> Result<(StatusCode, Json<PortalLeadResponse>), ApiError> {
    let lead = validate_lead_input(&body)?;
    let id = Uuid::now_v7();
    infra_postgres::commerces::insert_portal_lead(
        &state.app_pool,
        tenant_id,
        infra_postgres::commerces::PortalLeadInsert {
            id,
            contact_name: &lead.contact_name,
            phone: &lead.phone,
            commerce_name: &lead.commerce_name,
            email: &lead.email,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    Ok((
        StatusCode::CREATED,
        Json(PortalLeadResponse {
            id,
            contact_name: lead.contact_name,
            phone: lead.phone,
            commerce_name: lead.commerce_name,
            email: lead.email,
            status: "pending".into(),
            created_at: chrono::Utc::now().to_rfc3339(),
            reviewed_at: None,
        }),
    ))
}

pub async fn list_portal_leads(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<ListPortalLeadsQuery>,
) -> Result<Json<Vec<PortalLeadResponse>>, ApiError> {
    require_admin(&auth)?;
    let status = match query.status.as_deref() {
        None | Some("") => None,
        Some("pending" | "approved" | "rejected") => query.status.as_deref(),
        Some(_) => {
            return Err(ApiError::bad_request(
                "INVALID_INPUT",
                "status must be pending, approved, or rejected",
            ));
        }
    };
    let rows =
        infra_postgres::commerces::list_portal_leads(&state.app_pool, auth.tenant_id, status, 100)
            .await
            .map_err(|_| ApiError::internal())?;
    Ok(Json(rows.into_iter().map(map_lead).collect()))
}

pub async fn review_portal_lead(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<ReviewPortalLeadRequest>,
) -> Result<Json<PortalLeadResponse>, ApiError> {
    require_admin(&auth)?;
    let status = body.status.trim();
    if status != "approved" && status != "rejected" {
        return Err(ApiError::bad_request(
            "INVALID_INPUT",
            "status must be approved or rejected",
        ));
    }
    let row = infra_postgres::commerces::update_portal_lead_status(
        &state.app_pool,
        auth.tenant_id,
        id,
        status,
        auth.user_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::not_found)?;
    Ok(Json(map_lead(row)))
}

struct ValidLead {
    contact_name: String,
    phone: String,
    commerce_name: String,
    email: String,
}

fn validate_lead_input(body: &CreatePortalLeadRequest) -> Result<ValidLead, ApiError> {
    let contact_name = body.contact_name.trim().to_string();
    let commerce_name = body.commerce_name.trim().to_string();
    let email = body.email.trim().to_lowercase();
    if contact_name.is_empty() || contact_name.len() > 200 {
        return Err(ApiError::bad_request(
            "INVALID_INPUT",
            "contactName is required",
        ));
    }
    if commerce_name.is_empty() || commerce_name.len() > 200 {
        return Err(ApiError::bad_request(
            "INVALID_INPUT",
            "commerceName is required",
        ));
    }
    if !email.contains('@') || email.len() > 320 {
        return Err(ApiError::bad_request("INVALID_INPUT", "email is invalid"));
    }
    let phone: String = body.phone.chars().filter(|c| c.is_ascii_digit()).collect();
    if phone.len() < 10 || phone.len() > 15 {
        return Err(ApiError::bad_request(
            "INVALID_INPUT",
            "phone must be 10–15 digits",
        ));
    }
    Ok(ValidLead {
        contact_name,
        phone,
        commerce_name,
        email,
    })
}

fn map_lead(row: infra_postgres::commerces::PortalLeadRow) -> PortalLeadResponse {
    PortalLeadResponse {
        id: row.id,
        contact_name: row.contact_name,
        phone: row.phone,
        commerce_name: row.commerce_name,
        email: row.email,
        status: row.status,
        created_at: row.created_at.to_rfc3339(),
        reviewed_at: row.reviewed_at.map(|t| t.to_rfc3339()),
    }
}
