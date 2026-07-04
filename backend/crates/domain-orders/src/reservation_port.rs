use std::collections::HashMap;

use domain_inventory::ProductId;

/// Port for stock availability during order approval (RN2).
pub trait StockReservationPort {
    fn balance_and_reserved(&mut self, product_id: ProductId) -> (i32, i32);
    fn record_reservation(&mut self, product_id: ProductId, quantity: i32);
}

/// In-memory stock snapshot for domain tests and application orchestration.
#[derive(Debug, Clone, Default)]
pub struct StockSnapshot {
    balance_by_product: HashMap<ProductId, i32>,
    reserved_by_product: HashMap<ProductId, i32>,
}

impl StockSnapshot {
    pub fn new(
        balance_by_product: HashMap<ProductId, i32>,
        reserved_by_product: HashMap<ProductId, i32>,
    ) -> Self {
        Self {
            balance_by_product,
            reserved_by_product,
        }
    }

    pub fn with_balance(mut self, product_id: ProductId, balance: i32) -> Self {
        self.balance_by_product.insert(product_id, balance);
        self
    }

    pub fn with_reserved(mut self, product_id: ProductId, reserved: i32) -> Self {
        self.reserved_by_product.insert(product_id, reserved);
        self
    }
}

/// Tracks reservations made during a single approve call.
#[derive(Debug, Default)]
pub struct InMemoryReservationPort {
    snapshot: StockSnapshot,
    pending_reserved: HashMap<ProductId, i32>,
}

impl InMemoryReservationPort {
    pub fn new(snapshot: StockSnapshot) -> Self {
        Self {
            snapshot,
            pending_reserved: HashMap::new(),
        }
    }

    pub fn pending_reserved(&self) -> &HashMap<ProductId, i32> {
        &self.pending_reserved
    }
}

impl StockReservationPort for InMemoryReservationPort {
    fn balance_and_reserved(&mut self, product_id: ProductId) -> (i32, i32) {
        let balance = self
            .snapshot
            .balance_by_product
            .get(&product_id)
            .copied()
            .unwrap_or(0);
        let base_reserved = self
            .snapshot
            .reserved_by_product
            .get(&product_id)
            .copied()
            .unwrap_or(0);
        let extra = self.pending_reserved.get(&product_id).copied().unwrap_or(0);
        (balance, base_reserved + extra)
    }

    fn record_reservation(&mut self, product_id: ProductId, quantity: i32) {
        *self.pending_reserved.entry(product_id).or_insert(0) += quantity;
    }
}
