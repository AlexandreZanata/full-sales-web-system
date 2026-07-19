use crate::error::IdentityError;
use crate::public_code::normalize_public_code;
use crate::user_id::UserId;

pub struct SellerProfileInput {
    pub user_id: UserId,
    pub operating_region: Option<String>,
    pub monthly_target_amount: Option<i64>,
    pub public_code: Option<String>,
    pub contact_phone: Option<String>,
    pub share_link_active: bool,
}

/// Seller extension profile — 1:1 with User where role = Seller.
#[derive(Debug, Clone)]
pub struct SellerProfile {
    user_id: UserId,
    operating_region: Option<String>,
    monthly_target_amount: Option<i64>,
    public_code: Option<String>,
    contact_phone: Option<String>,
    share_link_active: bool,
}

impl SellerProfile {
    pub fn create(input: SellerProfileInput) -> Result<Self, IdentityError> {
        if let Some(amount) = input.monthly_target_amount
            && amount < 0
        {
            return Err(IdentityError::InvalidProfileField);
        }
        let public_code = match input.public_code {
            Some(raw) if raw.trim().is_empty() => None,
            Some(raw) => Some(normalize_public_code(&raw)?),
            None => None,
        };
        let contact_phone = match input.contact_phone {
            Some(raw) => normalize_contact_phone(&raw)?,
            None => None,
        };
        Ok(Self {
            user_id: input.user_id,
            operating_region: input
                .operating_region
                .map(|r| r.trim().to_owned())
                .filter(|r| !r.is_empty()),
            monthly_target_amount: input.monthly_target_amount,
            public_code,
            contact_phone,
            share_link_active: input.share_link_active,
        })
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn operating_region(&self) -> Option<&str> {
        self.operating_region.as_deref()
    }

    pub fn monthly_target_amount(&self) -> Option<i64> {
        self.monthly_target_amount
    }

    pub fn public_code(&self) -> Option<&str> {
        self.public_code.as_deref()
    }

    pub fn contact_phone(&self) -> Option<&str> {
        self.contact_phone.as_deref()
    }

    pub fn share_link_active(&self) -> bool {
        self.share_link_active
    }
}

/// Digits-only, 10–15 chars (same rules as tenant sales contact phone).
pub fn normalize_contact_phone(raw: &str) -> Result<Option<String>, IdentityError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let digits: String = trimmed.chars().filter(|ch| ch.is_ascii_digit()).collect();
    if digits.len() < 10 || digits.len() > 15 {
        return Err(IdentityError::InvalidProfileField);
    }
    Ok(Some(digits))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_id::UserId;
    use uuid::Uuid;

    fn uid() -> UserId {
        UserId::from_uuid(Uuid::nil())
    }

    #[test]
    fn given_phone_and_code_when_create_then_normalized() {
        let p = SellerProfile::create(SellerProfileInput {
            user_id: uid(),
            operating_region: None,
            monthly_target_amount: Some(100),
            public_code: Some("Maria-Silva".into()),
            contact_phone: Some("(11) 99999-8888".into()),
            share_link_active: true,
        })
        .unwrap();
        assert_eq!(p.public_code(), Some("maria-silva"));
        assert_eq!(p.contact_phone(), Some("11999998888"));
    }

    #[test]
    fn given_short_phone_when_create_then_err() {
        assert!(
            SellerProfile::create(SellerProfileInput {
                user_id: uid(),
                operating_region: None,
                monthly_target_amount: None,
                public_code: Some("maria".into()),
                contact_phone: Some("123".into()),
                share_link_active: true,
            })
            .is_err()
        );
    }
}
