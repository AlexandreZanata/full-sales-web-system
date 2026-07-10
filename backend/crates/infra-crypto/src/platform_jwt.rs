use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::jwt::JwtError;

pub const PLATFORM_ROLE: &str = "PlatformAdmin";
pub const MFA_PURPOSE: &str = "platform_mfa";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlatformAccessTokenClaims {
    pub sub: Uuid,
    pub role: String,
    pub exp: u64,
    #[serde(default)]
    pub impersonating: bool,
    #[serde(rename = "actingTenantId", skip_serializing_if = "Option::is_none")]
    pub acting_tenant_id: Option<Uuid>,
    #[serde(rename = "actingRole", skip_serializing_if = "Option::is_none")]
    pub acting_role: Option<String>,
    #[serde(rename = "grantId", skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<Uuid>,
    #[serde(rename = "actingUserId", skip_serializing_if = "Option::is_none")]
    pub acting_user_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MfaPendingClaims {
    pub sub: Uuid,
    pub purpose: String,
    pub exp: u64,
}

impl super::JwtService {
    pub fn issue_platform_access_token(
        &self,
        user_id: Uuid,
        impersonating: bool,
        acting_tenant_id: Option<Uuid>,
        acting_role: Option<&str>,
        grant_id: Option<Uuid>,
        acting_user_id: Option<Uuid>,
    ) -> Result<String, JwtError> {
        let exp = unix_expiry(self.ttl)?;
        let claims = PlatformAccessTokenClaims {
            sub: user_id,
            role: PLATFORM_ROLE.to_owned(),
            exp,
            impersonating,
            acting_tenant_id,
            acting_role: acting_role.map(str::to_owned),
            grant_id,
            acting_user_id,
        };
        encode_platform(&self.secret, &claims)
    }

    pub fn verify_platform_access_token(
        &self,
        token: &str,
    ) -> Result<PlatformAccessTokenClaims, JwtError> {
        decode::<PlatformAccessTokenClaims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| JwtError::DecodeFailed)
    }

    pub fn issue_mfa_pending_token(&self, user_id: Uuid) -> Result<String, JwtError> {
        let exp = unix_expiry(Duration::from_secs(300))?;
        let claims = MfaPendingClaims {
            sub: user_id,
            purpose: MFA_PURPOSE.to_owned(),
            exp,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .map_err(|_| JwtError::EncodeFailed)
    }

    pub fn verify_mfa_pending_token(&self, token: &str) -> Result<MfaPendingClaims, JwtError> {
        let claims = decode::<MfaPendingClaims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| JwtError::DecodeFailed)?;

        if claims.purpose != MFA_PURPOSE {
            return Err(JwtError::DecodeFailed);
        }
        Ok(claims)
    }

    pub fn issue_impersonation_token(
        &self,
        platform_user_id: Uuid,
        acting_tenant_id: Uuid,
        acting_user_id: Uuid,
        grant_id: Uuid,
        ttl: Duration,
    ) -> Result<String, JwtError> {
        let exp = unix_expiry(ttl)?;
        let claims = PlatformAccessTokenClaims {
            sub: platform_user_id,
            role: PLATFORM_ROLE.to_owned(),
            exp,
            impersonating: true,
            acting_tenant_id: Some(acting_tenant_id),
            acting_role: Some("Admin".to_owned()),
            grant_id: Some(grant_id),
            acting_user_id: Some(acting_user_id),
        };
        encode_platform(&self.secret, &claims)
    }
}

fn unix_expiry(ttl: Duration) -> Result<u64, JwtError> {
    SystemTime::now()
        .checked_add(ttl)
        .ok_or(JwtError::EncodeFailed)?
        .duration_since(UNIX_EPOCH)
        .map_err(|_| JwtError::EncodeFailed)
        .map(|d| d.as_secs())
}

fn encode_platform(secret: &[u8], claims: &PlatformAccessTokenClaims) -> Result<String, JwtError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(|_| JwtError::EncodeFailed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn given_platform_token_when_verify_then_roundtrip() {
        let jwt = super::super::JwtService::new("secret", Duration::from_secs(900));
        let user_id = Uuid::now_v7();
        let token = jwt
            .issue_platform_access_token(user_id, false, None, None, None, None)
            .expect("issue");
        let claims = jwt.verify_platform_access_token(&token).expect("verify");
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role, PLATFORM_ROLE);
        assert!(!claims.impersonating);
    }
}
