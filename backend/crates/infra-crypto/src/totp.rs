use totp_rs::{Algorithm, Secret, TOTP};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TotpError {
    #[error("invalid totp secret")]
    InvalidSecret,

    #[error("invalid totp code")]
    InvalidCode,
}

pub struct TotpVerifier {
    inner: TOTP,
}

impl TotpVerifier {
    pub fn from_base32_secret(secret: &str) -> Result<Self, TotpError> {
        let secret = Secret::Encoded(secret.to_owned())
            .to_bytes()
            .map_err(|_| TotpError::InvalidSecret)?;
        let inner = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret,
            Some("FullSales".to_owned()),
            "platform".to_owned(),
        )
            .map_err(|_| TotpError::InvalidSecret)?;
        Ok(Self { inner })
    }

    pub fn verify(&self, code: &str) -> Result<(), TotpError> {
        if self.inner.check_current(code).unwrap_or(false) {
            Ok(())
        } else {
            Err(TotpError::InvalidCode)
        }
    }

    pub fn current_code(&self) -> String {
        self.inner.generate_current().unwrap_or_default()
    }

    pub fn generate_secret_base32() -> String {
        Secret::generate_secret().to_encoded().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_known_secret_when_verify_current_code_then_ok() {
        let verifier =
            TotpVerifier::from_base32_secret("KVKFKRCPNZQUYMLXOVYDSQKJKZDTSRLD").expect("secret");
        let code = verifier.current_code();
        verifier.verify(&code).expect("valid");
    }
}
