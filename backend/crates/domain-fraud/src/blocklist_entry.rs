use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::FraudError;

#[derive(Debug, Clone)]
pub struct BlocklistEntry {
    pub id: Uuid,
    pub email: Option<String>,
    pub cnpj: Option<String>,
    pub ip: Option<String>,
    pub card_fingerprint: Option<String>,
    pub reason: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

impl BlocklistEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        email: Option<String>,
        cnpj: Option<String>,
        ip: Option<String>,
        card_fingerprint: Option<String>,
        reason: String,
        expires_at: Option<DateTime<Utc>>,
        created_by: Uuid,
    ) -> Result<Self, FraudError> {
        let has_identifier = email.as_ref().is_some_and(|v| !v.trim().is_empty())
            || cnpj.as_ref().is_some_and(|v| !v.trim().is_empty())
            || ip.as_ref().is_some_and(|v| !v.trim().is_empty())
            || card_fingerprint
                .as_ref()
                .is_some_and(|v| !v.trim().is_empty());
        if !has_identifier || reason.trim().is_empty() {
            return Err(FraudError::InvalidBlocklistEntry);
        }
        Ok(Self {
            id,
            email: email
                .map(|v| v.trim().to_lowercase())
                .filter(|v| !v.is_empty()),
            cnpj: cnpj.map(|v| v.chars().filter(|c| c.is_ascii_digit()).collect()),
            ip: ip.map(|v| v.trim().to_owned()).filter(|v| !v.is_empty()),
            card_fingerprint: card_fingerprint
                .map(|v| v.trim().to_owned())
                .filter(|v| !v.is_empty()),
            reason: reason.trim().to_owned(),
            expires_at,
            created_by,
            created_at: Utc::now(),
        })
    }

    pub fn is_active(&self, now: DateTime<Utc>) -> bool {
        self.expires_at.is_none_or(|expires| expires > now)
    }
}
