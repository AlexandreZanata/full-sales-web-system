use domain_commerces::Cnpj;
use serde::Deserialize;

use crate::auth::AuthUser;
use crate::cnpj_lookup::CnpjLookupError;
use crate::commerces::registrations::access::load_review_flag;
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CnpjLookupQuery {
    pub cnpj: String,
}

pub async fn lookup_cnpj(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
    axum::extract::Query(query): axum::extract::Query<CnpjLookupQuery>,
) -> Result<axum::Json<crate::cnpj_lookup::CnpjLookupResult>, ApiError> {
    let review_flag = load_review_flag(&state, &auth).await?;
    if !auth.role.can_submit_commerce() && !application::can_review_commerce(auth.role, review_flag)
    {
        return Err(ApiError::forbidden());
    }

    if Cnpj::parse(&query.cnpj).is_err() {
        return Err(ApiError::invalid_cnpj());
    }
    let digits: String = query.cnpj.chars().filter(|c| c.is_ascii_digit()).collect();
    let key = format!("cnpj-lookup:{}:{}", auth.tenant_id.as_uuid(), auth.user_id);
    if !state
        .rate_limiter
        .try_consume(&key, state.cnpj_lookup_rate_limit)
    {
        return Err(ApiError::rate_limited());
    }

    match state.cnpj_lookup.lookup(&digits).await {
        Ok(result) => Ok(axum::Json(result)),
        Err(CnpjLookupError::NotFound) => Err(ApiError::cnpj_not_found()),
        Err(CnpjLookupError::Unavailable) => Err(ApiError::cnpj_lookup_unavailable()),
    }
}
