use argon2::{
    Argon2, PasswordHash, PasswordHasher as _, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("password hashing failed")]
    HashFailed,

    #[error("invalid password hash")]
    InvalidHash,
}

/// Argon2id password hasher (SECURITY.md).
pub struct PasswordHasher;

impl PasswordHasher {
    pub fn hash(plain: &str) -> Result<String, PasswordError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(plain.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| PasswordError::HashFailed)
    }

    pub fn verify(plain: &str, encoded: &str) -> Result<bool, PasswordError> {
        let parsed = PasswordHash::new(encoded).map_err(|_| PasswordError::InvalidHash)?;
        Ok(Argon2::default()
            .verify_password(plain.as_bytes(), &parsed)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_password_when_hash_and_verify_then_matches() {
        let hash = PasswordHasher::hash("secret-password").expect("hash");
        assert!(PasswordHasher::verify("secret-password", &hash).expect("verify"));
        assert!(!PasswordHasher::verify("wrong", &hash).expect("verify wrong"));
    }
}
