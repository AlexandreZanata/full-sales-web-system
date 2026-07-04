use domain_inventory::{ProductId, Quantity};
use domain_shared::Money;

/// Single product line on a Sale — quantity, unit price, line subtotal.
#[derive(Debug, Clone)]
pub struct SaleItem {
    product_id: ProductId,
    quantity: Quantity,
    unit_price: Money,
    line_total: Money,
}

impl SaleItem {
    pub fn create(
        product_id: ProductId,
        quantity: Quantity,
        unit_price: Money,
    ) -> Result<Self, domain_shared::DomainError> {
        let line_total = unit_price
            .amount_minor()
            .checked_mul(quantity.value() as i64)
            .and_then(|amount| Money::new(amount, unit_price.currency()).ok())
            .ok_or(domain_shared::DomainError::MoneyOverflow)?;
        Ok(Self {
            product_id,
            quantity,
            unit_price,
            line_total,
        })
    }

    pub fn product_id(&self) -> ProductId {
        self.product_id
    }

    pub fn quantity(&self) -> Quantity {
        self.quantity
    }

    pub fn unit_price(&self) -> &Money {
        &self.unit_price
    }

    pub fn line_total(&self) -> &Money {
        &self.line_total
    }
}
