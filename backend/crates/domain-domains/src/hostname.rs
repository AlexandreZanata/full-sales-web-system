use crate::error::DomainError;

pub fn normalize_hostname(raw: &str) -> Result<String, DomainError> {
    let hostname = raw.trim().trim_end_matches('.').to_lowercase();
    if hostname.is_empty() || hostname.len() > 253 {
        return Err(DomainError::InvalidHostname);
    }
    if hostname.contains("..") || hostname.starts_with('-') || hostname.ends_with('-') {
        return Err(DomainError::InvalidHostname);
    }
    let labels: Vec<&str> = hostname.split('.').collect();
    if labels.len() < 2 {
        return Err(DomainError::InvalidHostname);
    }
    for label in &labels {
        if label.is_empty() || label.len() > 63 {
            return Err(DomainError::InvalidHostname);
        }
        if !label
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Err(DomainError::InvalidHostname);
        }
    }
    Ok(hostname)
}

pub fn is_reserved_hostname(hostname: &str, reserved: &[String]) -> bool {
    reserved.iter().any(|r| hostname == r || hostname.ends_with(&format!(".{r}")))
}

pub fn txt_record_name(hostname: &str) -> String {
    format!("_fullsales-verify.{hostname}")
}
