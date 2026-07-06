use axum::{
    Json,
    extract::{Path, RawQuery, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain_identity::Role;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin, require_roles};
use crate::commerces::addresses_support::{
    AddressResponse, address_response_from_row, ensure_commerce, load_commerce,
    load_existing_addresses, map_address_error,
};
use crate::error::ApiError;
use crate::list_query::{
    COMMERCE_ADDRESSES_LIST_CONFIG, CursorListResponse, build_cursor_page, decode_query_pairs,
    parse_list_query,
};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateAddressRequest {
    #[serde(rename = "addressType")]
    pub address_type: String,
    pub street: String,
    pub number: String,
    pub district: Option<String>,
    pub city: String,
    pub state: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(rename = "isPrimary", default)]
    pub is_primary: bool,
}

pub async fn list_addresses(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(commerce_id): Path<Uuid>,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<AddressResponse>>, Response> {
    require_roles(&auth, &[Role::Admin, Role::Driver, Role::Seller])
        .map_err(IntoResponse::into_response)?;
    ensure_commerce(&state, auth.tenant_id, commerce_id)
        .await
        .map_err(IntoResponse::into_response)?;
    let parsed = parse_list_query(
        &decode_query_pairs(query.as_deref()),
        &COMMERCE_ADDRESSES_LIST_CONFIG,
    )
    .map_err(IntoResponse::into_response)?;
    let rows = infra_postgres::commerces::addresses::list_addresses_by_commerce_cursor(
        &state.app_pool,
        auth.tenant_id,
        commerce_id,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    Ok(Json(build_cursor_page(
        rows.iter().map(address_response_from_row).collect(),
        parsed.pagination.limit,
        |address| address.id,
    )))
}

pub async fn create_address(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(commerce_id): Path<Uuid>,
    Json(body): Json<CreateAddressRequest>,
) -> Result<(StatusCode, Json<AddressResponse>), ApiError> {
    require_admin(&auth)?;
    let commerce = load_commerce(&state, auth.tenant_id, commerce_id).await?;
    let existing = load_existing_addresses(&state, auth.tenant_id, commerce_id).await?;
    let address_id = domain_commerces::CommerceAddressId::generate();
    let address_type: domain_commerces::AddressType = body
        .address_type
        .parse()
        .map_err(|_| ApiError::bad_request("INVALID_ADDRESS_TYPE", "Invalid address type"))?;

    let created = domain_commerces::CommerceAddress::create(
        &commerce,
        domain_commerces::CreateCommerceAddressInput {
            id: address_id,
            tenant_id: auth.tenant_id,
            commerce_id: commerce.id(),
            address_type,
            street: body.street,
            number: body.number,
            district: body.district,
            city: body.city,
            state: body.state,
            postal_code: body.postal_code,
            latitude: body.latitude,
            longitude: body.longitude,
            is_primary: body.is_primary,
        },
        &existing,
    )
    .map_err(map_address_error)?;

    infra_postgres::commerces::addresses::insert_address(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::commerces::addresses::AddressInsert {
            id: address_id.as_uuid(),
            commerce_id,
            address_type: address_type.as_str().to_owned(),
            street: created.street().to_owned(),
            number: created.number().to_owned(),
            district: created.district().map(str::to_owned),
            city: created.city().to_owned(),
            state: created.state().as_str().to_owned(),
            postal_code: created.postal_code().as_str().to_owned(),
            latitude: created.latitude(),
            longitude: created.longitude(),
            is_primary: created.is_primary(),
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let row = infra_postgres::commerces::addresses::find_address_by_id(
        &state.app_pool,
        auth.tenant_id,
        address_id.as_uuid(),
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::internal)?;

    Ok((StatusCode::CREATED, Json(address_response_from_row(&row))))
}
