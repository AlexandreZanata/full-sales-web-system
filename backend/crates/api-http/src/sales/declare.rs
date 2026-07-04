use axum::{
    Json,
    extract::{Path, State},
};
use chrono::Utc;
use domain_commerces::CommerceId;
use domain_identity::{Role, UserId};
use domain_orders::OrderId;
use domain_sales::{
    DeclaredPayment, DeclaredPaymentMethod, InMemoryPaymentDeclarationAuditPort, PaymentMethod,
    Sale, SaleId,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::sales::query::get_sale;
use crate::sales::types::{map_sales_app_error, parse_sale_status, SaleResponse};
use crate::session::session_from_auth;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct DeclarePaymentRequest {
    pub method: String,
    pub received: bool,
    pub notes: Option<String>,
}

pub async fn declare_sale_payment(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<DeclarePaymentRequest>,
) -> Result<Json<SaleResponse>, ApiError> {
    if auth.role != Role::Driver {
        return Err(ApiError::forbidden());
    }

    let row = infra_postgres::sales::find_sale_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::sale_not_found)?;

    if row.driver_id != auth.user_id {
        return Err(ApiError::unauthorized_payment_declaration());
    }

    let method = DeclaredPaymentMethod::parse(&body.method)
        .map_err(|_| ApiError::bad_request("VALIDATION_ERROR", "Invalid declared payment method"))?;

    let payment_method =
        PaymentMethod::parse(&row.payment_method).map_err(|_| ApiError::internal())?;
    let status = parse_sale_status(&row.status)?;
    let sale = Sale::restore(
        SaleId::from_uuid(row.id),
        UserId::from_uuid(row.driver_id),
        CommerceId::from_uuid(row.commerce_id),
        row.order_id.map(OrderId::from_uuid),
        payment_method,
        DeclaredPayment::not_declared(),
        auth.tenant_id,
        status,
        vec![],
    );

    let declared_at = Utc::now();
    let mut audit = InMemoryPaymentDeclarationAuditPort::default();
    let _updated = application::sales::declare_sale_payment(
        sale,
        application::sales::DeclareSalePaymentCommand {
            method,
            received: body.received,
            declared_at,
            declaring_user: UserId::from_uuid(auth.user_id),
            notes: body.notes.clone(),
        },
        &mut audit,
    )
    .map_err(map_sales_app_error)?;

    let session = session_from_auth(&auth);
    infra_postgres::sales::declare_payment(
        &state.app_pool,
        &session,
        &infra_postgres::sales::DeclarePaymentUpdate {
            sale_id: id,
            driver_id: auth.user_id,
            method: method.as_str().to_owned(),
            received: body.received,
            declared_at,
            notes: body.notes,
        },
    )
    .await
    .map_err(|err| match err {
        infra_postgres::PostgresError::Database(_) => ApiError::sale_not_found(),
        _ => ApiError::internal(),
    })?;

    get_sale(State(state), auth, Path(id)).await
}
