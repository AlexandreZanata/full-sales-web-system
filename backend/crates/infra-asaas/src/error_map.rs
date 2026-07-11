use domain_billing::BillingError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AsaasErrorBody {
    pub errors: Option<Vec<AsaasErrorItem>>,
}

#[derive(Debug, Deserialize)]
pub struct AsaasErrorItem {
    pub code: Option<String>,
    pub description: Option<String>,
}

pub fn map_status(status: reqwest::StatusCode, body: &str) -> BillingError {
    let parsed: Option<AsaasErrorBody> = serde_json::from_str(body).ok();
    let code = parsed
        .as_ref()
        .and_then(|b| b.errors.as_ref())
        .and_then(|e| e.first())
        .and_then(|e| e.code.as_deref());
    match status.as_u16() {
        401 | 403 => BillingError::InvalidCredentials,
        404 => BillingError::CustomerNotFound,
        429 => BillingError::RateLimited,
        400 | 422 => BillingError::InvalidRequest(code.unwrap_or("invalid_request").to_owned()),
        _ if status.is_server_error() => BillingError::UpstreamUnavailable,
        _ => BillingError::UpstreamUnavailable,
    }
}
