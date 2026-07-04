use domain_identity::{IdentityError, Role};
use uuid::Uuid;

use crate::{AppError, AuthenticatedUser};

/// Credential row loaded from persistence for login.
#[derive(Debug, Clone)]
pub struct LoginUserRecord {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub role: String,
    pub password_hash: String,
    pub active: bool,
}

/// BR-IA-002 + credential check — domain rules before token issue.
pub fn authenticate_login(
    record: &LoginUserRecord,
    _password: &str,
    password_matches: bool,
) -> Result<AuthenticatedUser, AppError> {
    if !password_matches {
        return Err(AppError::InvalidCredentials);
    }
    if !record.active {
        return Err(AppError::Identity(IdentityError::InactiveUser));
    }

    Ok(AuthenticatedUser {
        user_id: record.id,
        tenant_id: domain_shared::TenantId::from_uuid(record.tenant_id),
        role: Role::parse(&record.role)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(active: bool) -> LoginUserRecord {
        LoginUserRecord {
            id: Uuid::now_v7(),
            tenant_id: Uuid::now_v7(),
            role: "Driver".into(),
            password_hash: "hash".into(),
            active,
        }
    }

    #[test]
    fn br_ia_002_given_inactive_user_when_authenticate_then_inactive_user() {
        let result = authenticate_login(&record(false), "pw", true);
        assert!(matches!(
            result,
            Err(AppError::Identity(IdentityError::InactiveUser))
        ));
    }

    #[test]
    fn given_wrong_password_when_authenticate_then_invalid_credentials() {
        let result = authenticate_login(&record(true), "pw", false);
        assert!(matches!(result, Err(AppError::InvalidCredentials)));
    }
}
