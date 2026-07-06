use axum::{
    Json,
    extract::{Path, RawQuery, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain_commerces::{
    CommerceAddressId, CreateCommerceAddressInput, RegistrationMode, RegistrationStatus,
};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::list_query::{
    CursorListResponse, REGISTRATIONS_LIST_CONFIG, build_cursor_page, decode_query_pairs,
    filter_eq_string, parse_list_query,
};
use crate::state::AppState;

pub(crate) mod access;
mod review;
mod types;

pub use review::{approve_registration, patch_registration, reject_registration};
pub use types::SubmitRegistrationRequest;

use access::{ensure_registration_readable, load_review_flag};
use types::{map_commerce_domain_error, map_registration_error, registration_response_from_row};

pub async fn submit_registration(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<SubmitRegistrationRequest>,
) -> Result<(StatusCode, Json<types::RegistrationResponse>), ApiError> {
    application::ensure_can_submit_commerce(auth.role).map_err(map_registration_error)?;

    if infra_postgres::commerces::registrations::commerce_cnpj_exists(
        &state.app_pool,
        auth.tenant_id,
        body.cnpj.trim(),
    )
    .await
    .map_err(|_| ApiError::internal())?
    {
        return Err(ApiError::cnpj_already_registered());
    }

    let commerce = application::parse_submit_commerce_registration(
        &body.cnpj,
        &body.legal_name,
        body.trade_name.as_deref(),
        auth.tenant_id,
        auth.user_id,
        &body.registration_mode,
    )
    .map_err(map_registration_error)?;

    let address_id = CommerceAddressId::generate();
    let restored = application::restore_commerce_with_status(
        commerce.id().as_uuid(),
        commerce.cnpj().as_str(),
        commerce.legal_name(),
        commerce.trade_name(),
        auth.tenant_id,
        false,
        RegistrationStatus::PendingReview,
        Some(auth.user_id),
        commerce.registration_mode(),
    )
    .map_err(map_commerce_domain_error)?;

    let address = domain_commerces::CommerceAddress::create(
        &restored,
        CreateCommerceAddressInput {
            id: address_id,
            tenant_id: auth.tenant_id,
            commerce_id: commerce.id(),
            address_type: domain_commerces::AddressType::Delivery,
            street: body.delivery_address.street,
            number: body.delivery_address.number,
            district: body.delivery_address.district,
            city: body.delivery_address.city,
            state: body.delivery_address.state,
            postal_code: body.delivery_address.postal_code,
            latitude: None,
            longitude: None,
            is_primary: body.delivery_address.is_primary,
        },
        &[],
    )
    .map_err(|_| ApiError::bad_request("INVALID_ADDRESS", "Invalid delivery address"))?;

    let mut legacy_address = serde_json::json!({
        "street": address.street(),
        "number": address.number(),
        "city": address.city(),
        "state": address.state().as_str(),
        "postalCode": address.postal_code().as_str(),
        "contact": {
            "phone": body.contact.phone,
            "email": body.contact.email,
        }
    });
    if let Some(district) = address.district() {
        legacy_address["district"] = district.into();
    }

    let mode = commerce
        .registration_mode()
        .map(RegistrationMode::as_str)
        .unwrap_or("manual");

    infra_postgres::commerces::registrations::insert_registration(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::commerces::registrations::RegistrationInsert {
            commerce: infra_postgres::commerces::CommerceInsert {
                id: commerce.id().as_uuid(),
                cnpj: commerce.cnpj().as_str(),
                legal_name: commerce.legal_name(),
                trade_name: commerce.trade_name(),
                address: legacy_address,
                active: false,
                registration_status: "PendingReview",
                submitted_by_user_id: Some(auth.user_id),
                reviewed_by_user_id: None,
                rejection_reason: None,
                lookup_snapshot: body.lookup_snapshot,
                registration_mode: Some(mode),
            },
            delivery_address: infra_postgres::commerces::addresses::AddressInsert {
                id: address_id.as_uuid(),
                commerce_id: commerce.id().as_uuid(),
                address_type: "Delivery".into(),
                street: address.street().to_owned(),
                number: address.number().to_owned(),
                district: address.district().map(str::to_owned),
                city: address.city().to_owned(),
                state: address.state().as_str().to_owned(),
                postal_code: address.postal_code().as_str().to_owned(),
                latitude: None,
                longitude: None,
                is_primary: address.is_primary(),
            },
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    reload_registration(&state, auth.tenant_id, commerce.id().as_uuid())
        .await
        .map(|row| (StatusCode::CREATED, Json(row)))
}

pub async fn list_registrations(
    State(state): State<AppState>,
    auth: AuthUser,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<types::RegistrationResponse>>, Response> {
    let review_flag = load_review_flag(&state, &auth)
        .await
        .map_err(IntoResponse::into_response)?;
    let is_reviewer = application::can_review_commerce(auth.role, review_flag);
    if !auth.role.can_submit_commerce() && !is_reviewer {
        return Err(IntoResponse::into_response(ApiError::forbidden()));
    }

    let parsed = parse_list_query(
        &decode_query_pairs(query.as_deref()),
        &REGISTRATIONS_LIST_CONFIG,
    )
    .map_err(IntoResponse::into_response)?;
    let status = filter_eq_string(&parsed.filters, "status");
    let submitted_by = if is_reviewer {
        None
    } else {
        Some(auth.user_id)
    };

    let rows = infra_postgres::commerces::registrations::list_registrations_cursor(
        &state.app_pool,
        auth.tenant_id,
        status.as_deref(),
        submitted_by,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let items: Vec<_> = rows.iter().map(registration_response_from_row).collect();
    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |item| item.id,
    )))
}

pub async fn get_registration(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<types::RegistrationResponse>, ApiError> {
    let row = infra_postgres::commerces::registrations::find_commerce_registration_by_id(
        &state.app_pool,
        auth.tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::commerce_not_found)?;
    ensure_registration_readable(&state, &auth, &row).await?;
    Ok(Json(registration_response_from_row(&row)))
}

pub(crate) async fn reload_registration(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    id: Uuid,
) -> Result<types::RegistrationResponse, ApiError> {
    let row = infra_postgres::commerces::registrations::find_commerce_registration_by_id(
        &state.app_pool,
        tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::commerce_not_found)?;
    Ok(registration_response_from_row(&row))
}
