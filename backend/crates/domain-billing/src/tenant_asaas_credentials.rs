use chrono::{DateTime, Utc};
use domain_shared::TenantId;

/// Connected tenant Asaas account metadata — API key stored encrypted outside domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantAsaasCredentials {
    pub tenant_id: TenantId,
    pub api_key_last4: String,
    pub connected_at: DateTime<Utc>,
}

impl TenantAsaasCredentials {
    pub fn new(
        tenant_id: TenantId,
        api_key_last4: String,
        connected_at: DateTime<Utc>,
    ) -> Result<Self, crate::BillingError> {
        if api_key_last4.len() != 4 {
            return Err(crate::BillingError::InvalidRequest(
                "invalid_api_key_last4".into(),
            ));
        }
        Ok(Self {
            tenant_id,
            api_key_last4,
            connected_at,
        })
    }
}
