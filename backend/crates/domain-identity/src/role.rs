use crate::error::IdentityError;

/// Authorization profile — Admin | Driver | Seller | CommerceContact (GLOSSARY).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Admin,
    Driver,
    Seller,
    CommerceContact,
}

impl Role {
    pub fn parse(value: &str) -> Result<Self, IdentityError> {
        match value {
            "Admin" => Ok(Self::Admin),
            "Driver" => Ok(Self::Driver),
            "Seller" => Ok(Self::Seller),
            "CommerceContact" => Ok(Self::CommerceContact),
            _ => Err(IdentityError::InvalidRole),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "Admin",
            Self::Driver => "Driver",
            Self::Seller => "Seller",
            Self::CommerceContact => "CommerceContact",
        }
    }

    pub fn can_register_commerce(self) -> bool {
        matches!(self, Self::Admin)
    }

    pub fn can_submit_commerce(self) -> bool {
        matches!(self, Self::Seller)
    }

    pub fn can_review_commerce_by_role(self) -> bool {
        matches!(self, Self::Admin)
    }
}
