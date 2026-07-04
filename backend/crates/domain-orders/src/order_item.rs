use domain_inventory::{ProductId, Quantity};
use domain_shared::Money;

use crate::order_item_id::OrderItemId;

/// Single product line on an Order — frozen unit price at creation (RN3).
#[derive(Debug, Clone)]
pub struct OrderItem {
    id: OrderItemId,
    product_id: ProductId,
    quantity_requested: Quantity,
    quantity_delivered: Option<Quantity>,
    unit_price: Money,
    line_total: Money,
}

impl OrderItem {
    pub fn create(
        id: OrderItemId,
        product_id: ProductId,
        quantity_requested: Quantity,
        unit_price: Money,
    ) -> Result<Self, domain_shared::DomainError> {
        let line_total = unit_price
            .amount_minor()
            .checked_mul(quantity_requested.value() as i64)
            .and_then(|amount| Money::new(amount, unit_price.currency()).ok())
            .ok_or(domain_shared::DomainError::MoneyOverflow)?;
        Ok(Self {
            id,
            product_id,
            quantity_requested,
            quantity_delivered: None,
            unit_price,
            line_total,
        })
    }

    pub fn id(&self) -> OrderItemId {
        self.id
    }

    pub fn product_id(&self) -> ProductId {
        self.product_id
    }

    pub fn quantity_requested(&self) -> Quantity {
        self.quantity_requested
    }

    pub fn quantity_delivered(&self) -> Option<Quantity> {
        self.quantity_delivered
    }

    pub fn unit_price(&self) -> &Money {
        &self.unit_price
    }

    pub fn line_total(&self) -> &Money {
        &self.line_total
    }

    pub fn restore(
        id: OrderItemId,
        product_id: ProductId,
        quantity_requested: Quantity,
        quantity_delivered: Option<Quantity>,
        unit_price: Money,
        line_total: Money,
    ) -> Self {
        Self {
            id,
            product_id,
            quantity_requested,
            quantity_delivered,
            unit_price,
            line_total,
        }
    }
}
