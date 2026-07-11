use subtle::ConstantTimeEq;

pub const WEBHOOK_TOKEN_HEADER: &str = "asaas-access-token";
pub const WEBHOOK_TOKEN_ENV: &str = "ASAAS_WEBHOOK_TOKEN";

pub fn webhook_token_from_env() -> Option<String> {
    std::env::var(WEBHOOK_TOKEN_ENV)
        .ok()
        .filter(|t| !t.trim().is_empty())
}

pub fn validate_webhook_token(provided: Option<&str>, expected: &str) -> bool {
    let provided = provided.unwrap_or("");
    provided.as_bytes().ct_eq(expected.as_bytes()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_matching_token_when_validate_then_true() {
        assert!(validate_webhook_token(Some("secret-token"), "secret-token"));
    }

    #[test]
    fn given_wrong_token_when_validate_then_false() {
        assert!(!validate_webhook_token(Some("wrong"), "secret-token"));
    }
}
