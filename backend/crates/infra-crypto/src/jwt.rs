use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("jwt encode failed")]
    EncodeFailed,

    #[error("jwt decode failed")]
    DecodeFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub tenant_id: Uuid,
    pub role: String,
    #[serde(rename = "commerceId", skip_serializing_if = "Option::is_none")]
    pub commerce_id: Option<Uuid>,
    pub exp: u64,
}

#[derive(Clone)]
pub struct JwtService {
    secret: Vec<u8>,
    ttl: Duration,
}

impl JwtService {
    pub fn new(secret: impl Into<Vec<u8>>, ttl: Duration) -> Self {
        Self {
            secret: secret.into(),
            ttl,
        }
    }

    pub fn access_ttl_secs(&self) -> u64 {
        self.ttl.as_secs()
    }

    pub fn issue_access_token(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        role: &str,
        commerce_id: Option<Uuid>,
    ) -> Result<String, JwtError> {
        let exp = SystemTime::now()
            .checked_add(self.ttl)
            .ok_or(JwtError::EncodeFailed)?
            .duration_since(UNIX_EPOCH)
            .map_err(|_| JwtError::EncodeFailed)?
            .as_secs();

        let claims = AccessTokenClaims {
            sub: user_id,
            tenant_id,
            role: role.to_owned(),
            commerce_id,
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .map_err(|_| JwtError::EncodeFailed)
    }

    pub fn verify_access_token(&self, token: &str) -> Result<AccessTokenClaims, JwtError> {
        decode::<AccessTokenClaims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| JwtError::DecodeFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_claims_when_issue_and_verify_then_roundtrip() {
        let jwt = JwtService::new("test-secret", Duration::from_secs(900));
        let user_id = Uuid::now_v7();
        let tenant_id = Uuid::now_v7();
        let token = jwt
            .issue_access_token(user_id, tenant_id, "Admin", None)
            .expect("issue");
        let claims = jwt.verify_access_token(&token).expect("verify");
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.tenant_id, tenant_id);
        assert_eq!(claims.role, "Admin");
        assert!(claims.commerce_id.is_none());
    }

    #[test]
    fn given_commerce_contact_when_issue_token_then_commerce_id_in_claims() {
        let jwt = JwtService::new("test-secret", Duration::from_secs(900));
        let user_id = Uuid::now_v7();
        let tenant_id = Uuid::now_v7();
        let commerce_id = Uuid::now_v7();
        let token = jwt
            .issue_access_token(user_id, tenant_id, "CommerceContact", Some(commerce_id))
            .expect("issue");
        let claims = jwt.verify_access_token(&token).expect("verify");
        assert_eq!(claims.commerce_id, Some(commerce_id));
    }
}
