//! Password hashing (Argon2id) and JWT access tokens.

pub mod aes_gcm;
pub mod jwt;
pub mod platform_jwt;
pub mod password;
pub mod totp;

pub use aes_gcm::{AesGcmError, CredentialEncryptor, EncryptedBlob, MASTER_KEY_ENV};

pub use jwt::{AccessTokenClaims, JwtError, JwtService};
pub use platform_jwt::{
    MFA_PURPOSE, PLATFORM_ROLE, MfaPendingClaims, PlatformAccessTokenClaims,
};
pub use password::{PasswordError, PasswordHasher};
pub use totp::{TotpError, TotpVerifier};
