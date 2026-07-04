use crate::error::IdentityError;

/// Authorization profile — Admin | Driver | Seller (GLOSSARY).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Admin,
    Driver,
    Seller,
}

impl Role {
    pub fn parse(value: &str) -> Result<Self, IdentityError> {
        match value {
            "Admin" => Ok(Self::Admin),
            "Driver" => Ok(Self::Driver),
            "Seller" => Ok(Self::Seller),
            _ => Err(IdentityError::InvalidRole),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "Admin",
            Self::Driver => "Driver",
            Self::Seller => "Seller",
        }
    }

    pub fn can_register_commerce(self) -> bool {
        matches!(self, Self::Admin)
    }
}
