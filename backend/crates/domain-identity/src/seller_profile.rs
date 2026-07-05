use crate::error::IdentityError;
use crate::user_id::UserId;

pub struct SellerProfileInput {
    pub user_id: UserId,
    pub operating_region: Option<String>,
    pub monthly_target_amount: Option<i64>,
}

/// Seller extension profile — 1:1 with User where role = Seller.
#[derive(Debug, Clone)]
pub struct SellerProfile {
    user_id: UserId,
    operating_region: Option<String>,
    monthly_target_amount: Option<i64>,
}

impl SellerProfile {
    pub fn create(input: SellerProfileInput) -> Result<Self, IdentityError> {
        if let Some(amount) = input.monthly_target_amount {
            if amount < 0 {
                return Err(IdentityError::InvalidProfileField);
            }
        }
        Ok(Self {
            user_id: input.user_id,
            operating_region: input
                .operating_region
                .map(|r| r.trim().to_owned())
                .filter(|r| !r.is_empty()),
            monthly_target_amount: input.monthly_target_amount,
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
}
