use domain_shared::TenantId;

use crate::product_id::ProductId;
use crate::quantity::Quantity;

/// Tenant-level available stock per ADR-010 / DE-001.
///
/// ```text
/// available(product) =
///   SUM(stock_balances.quantity for tenant+product)
///   − SUM(stock_reservations.quantity_reserved WHERE status = Active)
/// ```
pub fn tenant_available_stock(
    balance_total: i32,
    active_reserved: i32,
) -> i32 {
    (balance_total - active_reserved).max(0)
}

pub struct AvailableStockInput {
    pub tenant_id: TenantId,
    pub product_id: ProductId,
    pub balance_total: i32,
    pub active_reserved: i32,
}

/// Returns how many units can still be reserved for portal orders.
pub fn compute_available(input: AvailableStockInput) -> i32 {
    let _ = (input.tenant_id, input.product_id);
    tenant_available_stock(input.balance_total, input.active_reserved)
}

/// Checks whether a reservation quantity fits within available stock.
pub fn ensure_can_reserve(available: i32, quantity: Quantity) -> bool {
    available >= quantity.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_balances_and_reservations_when_compute_then_available() {
        assert_eq!(tenant_available_stock(100, 30), 70);
        assert_eq!(tenant_available_stock(10, 15), 0);
    }
}
