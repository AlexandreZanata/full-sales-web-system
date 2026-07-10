use domain_shared::{Currency, Money};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::billing_interval::BillingInterval;
use crate::error::BillingError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub price: Money,
    pub billing_interval: BillingInterval,
    pub feature_limits: serde_json::Value,
    pub active: bool,
}

impl SubscriptionPlan {
    pub fn new(
        id: Uuid,
        code: &str,
        name: &str,
        price_minor: i64,
        billing_interval: BillingInterval,
        feature_limits: serde_json::Value,
    ) -> Result<Self, BillingError> {
        let code = code.trim();
        if code.is_empty() || code.len() > 32 {
            return Err(BillingError::InvalidPlanCode);
        }
        let price = Money::new(price_minor, Currency::brl())
            .map_err(|_| BillingError::InvalidRequest("invalid_plan_price".into()))?;
        Ok(Self {
            id,
            code: code.to_owned(),
            name: name.trim().to_owned(),
            price,
            billing_interval,
            feature_limits,
            active: true,
        })
    }

    pub fn price_major(&self) -> f64 {
        self.price.amount_minor() as f64 / 100.0
    }
}
