use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use thiserror::Error;

pub const MASTER_KEY_ENV: &str = "BILLING_CREDENTIALS_MASTER_KEY";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AesGcmError {
    #[error("master key not configured")]
    MasterKeyMissing,

    #[error("invalid master key encoding")]
    InvalidMasterKey,

    #[error("encryption failed")]
    EncryptFailed,

    #[error("decryption failed")]
    DecryptFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedBlob {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub key_version: i16,
}

/// AES-256-GCM encryptor for tenant Asaas API keys (ADR-018).
#[derive(Clone)]
pub struct CredentialEncryptor {
    cipher: Aes256Gcm,
    key_version: i16,
}

impl CredentialEncryptor {
    pub fn from_master_key_b64(
        master_key_b64: &str,
        key_version: i16,
    ) -> Result<Self, AesGcmError> {
        let key_bytes = STANDARD
            .decode(master_key_b64.trim())
            .map_err(|_| AesGcmError::InvalidMasterKey)?;
        if key_bytes.len() != 32 {
            return Err(AesGcmError::InvalidMasterKey);
        }
        let mut key = [0u8; 32];
        key.copy_from_slice(&key_bytes);
        Ok(Self {
            cipher: Aes256Gcm::new_from_slice(&key).map_err(|_| AesGcmError::InvalidMasterKey)?,
            key_version,
        })
    }

    pub fn from_env() -> Result<Self, AesGcmError> {
        let master_key =
            std::env::var(MASTER_KEY_ENV).map_err(|_| AesGcmError::MasterKeyMissing)?;
        Self::from_master_key_b64(&master_key, 1)
    }

    pub fn key_version(&self) -> i16 {
        self.key_version
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<EncryptedBlob, AesGcmError> {
        let nonce_bytes: [u8; 12] = rand::random();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|_| AesGcmError::EncryptFailed)?;
        Ok(EncryptedBlob {
            ciphertext,
            nonce: nonce_bytes.to_vec(),
            key_version: self.key_version,
        })
    }

    pub fn decrypt(&self, blob: &EncryptedBlob) -> Result<String, AesGcmError> {
        if blob.nonce.len() != 12 {
            return Err(AesGcmError::DecryptFailed);
        }
        let nonce = Nonce::from_slice(&blob.nonce);
        let plaintext = self
            .cipher
            .decrypt(nonce, blob.ciphertext.as_ref())
            .map_err(|_| AesGcmError::DecryptFailed)?;
        String::from_utf8(plaintext).map_err(|_| AesGcmError::DecryptFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_encryptor() -> CredentialEncryptor {
        let key = STANDARD.encode([7u8; 32]);
        CredentialEncryptor::from_master_key_b64(&key, 1).expect("encryptor")
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let enc = test_encryptor();
        let blob = enc.encrypt("sk_test_asaas_key_1234").expect("encrypt");
        let plain = enc.decrypt(&blob).expect("decrypt");
        assert_eq!(plain, "sk_test_asaas_key_1234");
    }
}
