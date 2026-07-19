//! Seller public share-link code (URL slug).

use crate::error::IdentityError;

const MIN_LEN: usize = 3;
const MAX_LEN: usize = 32;

const RESERVED: &[&str] = &[
    "products", "cart", "login", "orders", "s", "api", "admin", "settings", "me",
];

/// Normalizes and validates a seller public code (OD-19-1 A).
pub fn normalize_public_code(raw: &str) -> Result<String, IdentityError> {
    let lower = raw.trim().to_ascii_lowercase();
    if lower.len() < MIN_LEN || lower.len() > MAX_LEN {
        return Err(IdentityError::InvalidProfileField);
    }
    if !lower
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(IdentityError::InvalidProfileField);
    }
    if lower.starts_with('-') || lower.ends_with('-') || lower.contains("--") {
        return Err(IdentityError::InvalidProfileField);
    }
    if RESERVED.contains(&lower.as_str()) {
        return Err(IdentityError::InvalidProfileField);
    }
    Ok(lower)
}

/// Builds a slug candidate from a display name (backfill / seed helper).
pub fn slug_from_name(name: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in name.trim().chars() {
        let c = ch.to_ascii_lowercase();
        if c.is_ascii_alphanumeric() {
            out.push(c);
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.len() > 28 {
        out.truncate(28);
        while out.ends_with('-') {
            out.pop();
        }
    }
    if out.len() < MIN_LEN {
        "seller".into()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_valid_slug_when_normalize_then_ok() {
        assert_eq!(normalize_public_code("Maria").unwrap(), "maria");
    }

    #[test]
    fn given_reserved_when_normalize_then_err() {
        assert!(normalize_public_code("cart").is_err());
    }

    #[test]
    fn given_name_when_slug_then_kebab() {
        assert_eq!(slug_from_name("Dev Seller"), "dev-seller");
    }
}
