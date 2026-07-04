use ed25519_dalek::Signer;
use sha2::{Digest, Sha256};

/// Signs the SHA-256 digest of canonical payload bytes (BR-RE-001).
pub fn sign_canonical_payload(canonical: &str, signing_key: &ed25519_dalek::SigningKey) -> Vec<u8> {
    let digest = Sha256::digest(canonical.as_bytes());
    signing_key.sign(&digest).to_bytes().to_vec()
}

/// Verifies an Ed25519 signature over the SHA-256 digest of canonical payload bytes.
pub fn verify_canonical_payload(
    canonical: &str,
    signature: &[u8],
    verifying_key: &ed25519_dalek::VerifyingKey,
) -> bool {
    let Ok(sig) = ed25519_dalek::Signature::from_slice(signature) else {
        return false;
    };
    let digest = Sha256::digest(canonical.as_bytes());
    verifying_key.verify_strict(&digest, &sig).is_ok()
}
