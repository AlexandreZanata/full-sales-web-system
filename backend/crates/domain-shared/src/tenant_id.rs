use uuid::Uuid;

use crate::DomainError;

/// Tenant scope identifier for multi-tenancy and RLS (ADR-002).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TenantId(Uuid);

impl TenantId {
    /// Generates a new time-sortable tenant id (UUIDv7).
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Wraps an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }

    /// Parses a canonical UUID string.
    pub fn parse(input: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(input)
            .map(Self)
            .map_err(|_| DomainError::InvalidTenantId)
    }
}

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn given_generate_when_called_then_valid_uuid_v7() {
        let id = TenantId::generate();
        assert_eq!(id.as_uuid().get_version(), Some(uuid::Version::SortRand));
    }

    #[test]
    fn given_valid_uuid_string_when_parse_then_ok() {
        let raw = "550e8400-e29b-41d4-a716-446655440000";
        let id = TenantId::parse(raw).expect("valid uuid");
        assert_eq!(id.to_string(), raw);
    }

    #[test]
    fn given_uuid_when_from_uuid_then_wraps() {
        let raw = uuid::Uuid::now_v7();
        let id = TenantId::from_uuid(raw);
        assert_eq!(id.as_uuid(), raw);
    }

    #[test]
    fn given_tenant_id_when_display_then_uuid_string() {
        let raw = "550e8400-e29b-41d4-a716-446655440000";
        let id = TenantId::parse(raw).expect("valid uuid");
        assert_eq!(format!("{id}"), raw);
    }

    #[test]
    fn given_invalid_string_when_parse_then_invalid_tenant_id() {
        assert_eq!(
            TenantId::parse("not-a-uuid"),
            Err(DomainError::InvalidTenantId)
        );
    }
}
