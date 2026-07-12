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
    let first = parsed
        .as_ref()
        .and_then(|b| b.errors.as_ref())
        .and_then(|e| e.first());
    let code = first.and_then(|e| e.code.as_deref());
    let detail = first
        .and_then(|e| e.description.as_deref())
        .filter(|d| !d.is_empty());
    match status.as_u16() {
        401 | 403 => BillingError::InvalidCredentials,
        404 => BillingError::CustomerNotFound,
        429 => BillingError::RateLimited,
        400 | 422 => {
            let msg = match (code, detail) {
                (Some(c), Some(d)) => format!("{c}: {d}"),
                (Some(c), None) => c.to_owned(),
                (None, Some(d)) => d.to_owned(),
                (None, None) => "invalid_request".to_owned(),
            };
            BillingError::InvalidRequest(msg)
        }
        _ if status.is_server_error() => BillingError::UpstreamUnavailable,
        _ => BillingError::UpstreamUnavailable,
    }
}
