/// Masks secrets before structured logging (OWASP — no API keys in logs).
pub fn mask_api_key(key: &str) -> String {
    let trimmed = key.trim();
    if trimmed.len() <= 8 {
        return "***".to_owned();
    }
    format!("{}...{}", &trimmed[..4], &trimmed[trimmed.len() - 4..])
}

pub fn mask_card_token(token: &str) -> String {
    if token.len() <= 6 {
        return "***".to_owned();
    }
    format!("{}***", &token[..4])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_api_key_when_mask_then_prefix_suffix_only() {
        let masked = mask_api_key("$aact_YOUR_SANDBOX_KEY_12345");
        assert!(masked.starts_with("$aac"));
        assert!(masked.contains("..."));
        assert!(!masked.contains("SANDBOX_KEY"));
    }
}
