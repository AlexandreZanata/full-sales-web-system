use domain_shared::TenantId;
use uuid::Uuid;

use crate::available_stock::{AvailableStockInput, compute_available, ensure_can_reserve};
use crate::error::InventoryError;
use crate::product_id::ProductId;
use crate::quantity::Quantity;
use crate::reservation_id::ReservationId;
use crate::reservation_status::ReservationStatus;

pub struct CreateStockReservationInput {
    pub id: ReservationId,
    pub tenant_id: TenantId,
    pub order_id: Uuid,
    pub order_item_id: Uuid,
    pub product_id: ProductId,
    pub driver_id: Option<Uuid>,
    pub quantity: Quantity,
    pub balance_total: i32,
    pub active_reserved: i32,
}

/// Quantity held against tenant available stock when an Order is approved (RN2).
#[derive(Debug, Clone)]
pub struct StockReservation {
    id: ReservationId,
    tenant_id: TenantId,
    order_id: Uuid,
    order_item_id: Uuid,
    product_id: ProductId,
    driver_id: Option<Uuid>,
    quantity: Quantity,
    status: ReservationStatus,
}

impl StockReservation {
    /// Creates an Active reservation after checking tenant available stock.
    pub fn reserve(input: CreateStockReservationInput) -> Result<Self, InventoryError> {
        let available = compute_available(AvailableStockInput {
            tenant_id: input.tenant_id,
            product_id: input.product_id,
            balance_total: input.balance_total,
            active_reserved: input.active_reserved,
        });
        if !ensure_can_reserve(available, input.quantity) {
            return Err(InventoryError::InsufficientAvailableStock);
        }

        Ok(Self {
            id: input.id,
            tenant_id: input.tenant_id,
            order_id: input.order_id,
            order_item_id: input.order_item_id,
            product_id: input.product_id,
            driver_id: input.driver_id,
            quantity: input.quantity,
            status: ReservationStatus::Active,
        })
    }

    pub fn release(mut self) -> Result<Self, InventoryError> {
        if self.status != ReservationStatus::Active {
            return Err(InventoryError::InvalidReservationTransition {
                from: self.status,
                to: ReservationStatus::Released,
            });
        }
        self.status = ReservationStatus::Released;
        Ok(self)
    }

    pub fn consume(mut self) -> Result<Self, InventoryError> {
        if self.status != ReservationStatus::Active {
            return Err(InventoryError::InvalidReservationTransition {
                from: self.status,
                to: ReservationStatus::Consumed,
            });
        }
        self.status = ReservationStatus::Consumed;
        Ok(self)
    }

    pub fn id(&self) -> ReservationId {
        self.id
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn order_id(&self) -> Uuid {
        self.order_id
    }

    pub fn order_item_id(&self) -> Uuid {
        self.order_item_id
    }

    pub fn product_id(&self) -> ProductId {
        self.product_id
    }

    pub fn driver_id(&self) -> Option<Uuid> {
        self.driver_id
    }

    pub fn quantity(&self) -> Quantity {
        self.quantity
    }

    pub fn status(&self) -> ReservationStatus {
        self.status
    }
}
