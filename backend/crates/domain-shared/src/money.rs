use crate::DomainError;

/// ISO 4217 currency code (3 uppercase ASCII letters).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Currency(String);

impl Currency {
    /// Parses a 3-letter uppercase ISO 4217 code (e.g. `"BRL"`).
    pub fn parse(code: &str) -> Result<Self, DomainError> {
        if code.len() != 3 || !code.bytes().all(|b| b.is_ascii_uppercase()) {
            return Err(DomainError::InvalidCurrency);
        }
        Ok(Self(code.to_owned()))
    }

    /// Brazilian Real — primary currency for MVP.
    pub fn brl() -> Self {
        Self(String::from("BRL"))
    }

    /// Returns the currency code as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Monetary amount in minor units with currency — never floating point (GLOSSARY: Money).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Money {
    amount_minor: i64,
    currency: Currency,
}

impl Money {
    /// Creates money from minor units (e.g. centavos) and currency.
    pub fn new(amount_minor: i64, currency: Currency) -> Result<Self, DomainError> {
        if amount_minor < 0 {
            return Err(DomainError::NegativeMoneyAmount);
        }
        Ok(Self {
            amount_minor,
            currency,
        })
    }

    /// Minor-unit amount (e.g. centavos for BRL).
    pub fn amount_minor(&self) -> i64 {
        self.amount_minor
    }

    pub fn currency(&self) -> Currency {
        self.currency.clone()
    }

    /// Adds two amounts when currencies match (BR-SA-002: totals computed from items).
    pub fn try_add(self, other: Self) -> Result<Self, DomainError> {
        if self.currency != other.currency {
            return Err(DomainError::CurrencyMismatch(
                self.currency.as_str().to_owned(),
                other.currency.as_str().to_owned(),
            ));
        }
        let sum = self
            .amount_minor
            .checked_add(other.amount_minor)
            .ok_or(DomainError::MoneyOverflow)?;
        Self::new(sum, self.currency)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn brl() -> Currency {
        Currency::brl()
    }

    // Contract: GLOSSARY — Money uses minor units, never f64.
    #[test]
    fn given_valid_brl_minor_units_when_create_then_ok() {
        let money = Money::new(15_000, brl()).expect("valid money");
        assert_eq!(money.amount_minor(), 15_000);
        assert_eq!(money.currency(), brl());
    }

    #[test]
    fn given_negative_amount_when_create_then_negative_money_amount() {
        let result = Money::new(-1, brl());
        assert_eq!(result, Err(DomainError::NegativeMoneyAmount));
    }

    #[test]
    fn given_invalid_currency_code_when_parse_then_invalid_currency() {
        assert_eq!(Currency::parse("brl"), Err(DomainError::InvalidCurrency));
        assert_eq!(Currency::parse("US"), Err(DomainError::InvalidCurrency));
    }

    #[test]
    fn given_valid_currency_code_when_parse_then_ok() {
        let currency = Currency::parse("USD").expect("valid currency");
        assert_eq!(currency.as_str(), "USD");
    }

    #[test]
    fn given_brl_factory_when_called_then_brl_code() {
        assert_eq!(Currency::brl().as_str(), "BRL");
    }

    // Contract: BR-SA-002 — total computed from items only (15000 + 0 = 15000).
    #[test]
    fn given_two_line_items_when_add_then_total_from_items() {
        let line_a = Money::new(10_000, brl()).expect("line a");
        let line_b = Money::new(5_000, brl()).expect("line b");
        let total = line_a.try_add(line_b).expect("same currency");
        assert_eq!(total.amount_minor(), 15_000);
    }

    #[test]
    fn given_different_currencies_when_add_then_currency_mismatch() {
        let brl_money = Money::new(100, brl()).expect("brl");
        let usd = Money::new(100, Currency::parse("USD").expect("usd")).expect("usd money");
        assert_eq!(
            brl_money.try_add(usd),
            Err(DomainError::CurrencyMismatch("BRL".into(), "USD".into()))
        );
    }

    #[test]
    fn given_overflow_when_add_then_money_overflow() {
        let max = Money::new(i64::MAX, brl()).expect("max");
        let one = Money::new(1, brl()).expect("one");
        assert_eq!(max.try_add(one), Err(DomainError::MoneyOverflow));
    }
}
