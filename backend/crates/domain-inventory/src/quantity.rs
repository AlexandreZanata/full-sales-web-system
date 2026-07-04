use crate::error::InventoryError;

/// Positive integer quantity for sale lines and stock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quantity(i32);

impl Quantity {
    pub fn of(value: i32) -> Result<Self, InventoryError> {
        if value < 1 {
            return Err(InventoryError::InvalidQuantity);
        }
        Ok(Self(value))
    }

    pub fn value(self) -> i32 {
        self.0
    }
}
