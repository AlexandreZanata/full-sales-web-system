use domain_commerces::{Commerce, CommerceAddress, CommerceAddressId, validate_order_delivery_address};
use domain_identity::UserId;
use domain_inventory::{
    CreateStockReservationInput, InventoryError, Product, Quantity, ReservationId,
    StockReservation,
};
use domain_shared::{Money, TenantId};

use crate::error::OrderError;
use crate::order_id::OrderId;
use crate::order_item::OrderItem;
use crate::order_item_id::OrderItemId;
use crate::order_source::OrderSource;
use crate::order_status::OrderStatus;
use crate::reservation_port::StockReservationPort;

pub struct OrderCreateInput {
    pub id: OrderId,
    pub tenant_id: TenantId,
    pub commerce: Commerce,
    pub created_by: UserId,
    pub source: OrderSource,
    pub delivery_address: CommerceAddress,
    pub notes: Option<String>,
}

pub struct AddOrderItemInput {
    pub item_id: OrderItemId,
    pub product: Product,
    pub quantity: Quantity,
}

/// B2B purchase intent — approval and fulfillment lifecycle.
#[derive(Debug, Clone)]
pub struct Order {
    id: OrderId,
    tenant_id: TenantId,
    commerce_id: domain_commerces::CommerceId,
    created_by: UserId,
    source: OrderSource,
    delivery_address_id: CommerceAddressId,
    notes: Option<String>,
    status: OrderStatus,
    rejection_reason: Option<String>,
    items: Vec<OrderItem>,
}

impl Order {
    pub fn create(input: OrderCreateInput) -> Result<Self, OrderError> {
        validate_order_delivery_address(&input.commerce, &input.delivery_address)
            .map_err(map_commerce_error)?;
        Ok(Self {
            id: input.id,
            tenant_id: input.tenant_id,
            commerce_id: input.commerce.id(),
            created_by: input.created_by,
            source: input.source,
            delivery_address_id: input.delivery_address.id(),
            notes: input.notes.filter(|n| !n.trim().is_empty()),
            status: OrderStatus::Draft,
            rejection_reason: None,
            items: Vec::new(),
        })
    }

    pub fn id(&self) -> OrderId {
        self.id
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn commerce_id(&self) -> domain_commerces::CommerceId {
        self.commerce_id
    }

    pub fn created_by(&self) -> UserId {
        self.created_by
    }

    pub fn source(&self) -> OrderSource {
        self.source
    }

    pub fn delivery_address_id(&self) -> CommerceAddressId {
        self.delivery_address_id
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    pub fn status(&self) -> OrderStatus {
        self.status
    }

    pub fn rejection_reason(&self) -> Option<&str> {
        self.rejection_reason.as_deref()
    }

    pub fn items(&self) -> &[OrderItem] {
        &self.items
    }

    pub fn total(&self) -> Result<Money, domain_shared::DomainError> {
        let currency = domain_shared::Currency::brl();
        self.items
            .iter()
            .try_fold(Money::new(0, currency.clone())?, |sum, item| {
                sum.try_add(item.line_total().clone())
            })
    }

    pub fn add_item(mut self, input: AddOrderItemInput) -> Result<Self, OrderError> {
        if !self.status.allows_item_changes() {
            return Err(OrderError::InvalidTransition {
                from: self.status,
                to: self.status,
            });
        }
        if !input.product.is_active() {
            return Err(OrderError::InactiveProduct);
        }
        let item = OrderItem::create(
            input.item_id,
            input.product.id(),
            input.quantity,
            input.product.unit_price().clone(),
        )
        .map_err(|_| OrderError::EmptyOrder)?;
        self.items.push(item);
        Ok(self)
    }

    pub fn submit(mut self) -> Result<Self, OrderError> {
        self.transition_to(OrderStatus::PendingApproval)?;
        if self.items.is_empty() {
            return Err(OrderError::EmptyOrder);
        }
        Ok(self)
    }

    pub fn approve(
        mut self,
        port: &mut impl StockReservationPort,
    ) -> Result<(Self, Vec<StockReservation>), OrderError> {
        self.transition_to(OrderStatus::Approved)?;
        let mut reservations = Vec::with_capacity(self.items.len());
        for item in &self.items {
            let (balance, reserved) = port.balance_and_reserved(item.product_id());
            let reservation = StockReservation::reserve(CreateStockReservationInput {
                id: ReservationId::generate(),
                tenant_id: self.tenant_id,
                order_id: self.id.as_uuid(),
                order_item_id: item.id().as_uuid(),
                product_id: item.product_id(),
                driver_id: None,
                quantity: item.quantity_requested(),
                balance_total: balance,
                active_reserved: reserved,
            })
            .map_err(map_inventory_error)?;
            port.record_reservation(item.product_id(), item.quantity_requested().value());
            reservations.push(reservation);
        }
        Ok((self, reservations))
    }

    pub fn reject(mut self, reason: &str) -> Result<Self, OrderError> {
        if reason.trim().is_empty() {
            return Err(OrderError::RejectionReasonRequired);
        }
        self.transition_to(OrderStatus::Rejected)?;
        self.rejection_reason = Some(reason.trim().to_owned());
        Ok(self)
    }

    pub fn start_picking(mut self) -> Result<Self, OrderError> {
        self.transition_to(OrderStatus::Picking)?;
        Ok(self)
    }

    pub fn mark_in_transit(mut self) -> Result<Self, OrderError> {
        self.transition_to(OrderStatus::InTransit)?;
        Ok(self)
    }

    pub fn cancel(mut self) -> Result<Self, OrderError> {
        if !self.status.can_cancel() {
            return Err(OrderError::InvalidTransition {
                from: self.status,
                to: OrderStatus::Cancelled,
            });
        }
        self.status = OrderStatus::Cancelled;
        Ok(self)
    }

    pub fn restore(
        id: OrderId,
        tenant_id: TenantId,
        commerce_id: domain_commerces::CommerceId,
        created_by: UserId,
        source: OrderSource,
        delivery_address_id: CommerceAddressId,
        notes: Option<String>,
        status: OrderStatus,
        rejection_reason: Option<String>,
        items: Vec<OrderItem>,
    ) -> Self {
        Self {
            id,
            tenant_id,
            commerce_id,
            created_by,
            source,
            delivery_address_id,
            notes,
            status,
            rejection_reason,
            items,
        }
    }

    fn transition_to(&mut self, target: OrderStatus) -> Result<(), OrderError> {
        if !can_transition(self.status, target) {
            return Err(OrderError::InvalidTransition {
                from: self.status,
                to: target,
            });
        }
        self.status = target;
        Ok(())
    }
}

fn can_transition(from: OrderStatus, to: OrderStatus) -> bool {
    matches!(
        (from, to),
        (OrderStatus::Draft, OrderStatus::PendingApproval)
            | (OrderStatus::PendingApproval, OrderStatus::Approved)
            | (OrderStatus::PendingApproval, OrderStatus::Rejected)
            | (OrderStatus::Approved, OrderStatus::Picking)
            | (OrderStatus::Picking, OrderStatus::InTransit)
            | (OrderStatus::InTransit, OrderStatus::Delivered)
            | (OrderStatus::InTransit, OrderStatus::PartiallyDelivered)
    )
}

fn map_commerce_error(error: domain_commerces::CommerceError) -> OrderError {
    match error {
        domain_commerces::CommerceError::InactiveCommerce => OrderError::InactiveCommerce,
        domain_commerces::CommerceError::InvalidDeliveryAddress
        | domain_commerces::CommerceError::AddressCommerceMismatch => {
            OrderError::InvalidDeliveryAddress
        }
        _ => OrderError::InvalidDeliveryAddress,
    }
}

fn map_inventory_error(error: InventoryError) -> OrderError {
    match error {
        InventoryError::InsufficientAvailableStock => OrderError::InsufficientAvailableStock,
        _ => OrderError::InsufficientAvailableStock,
    }
}

#[cfg(test)]
mod tests {
    use domain_commerces::{
        AddressType, Cnpj, Commerce, CommerceAddress, CommerceAddressId, CommerceId,
        CreateCommerceAddressInput, CreateCommerceInput,
    };
    use domain_identity::UserId;
    use domain_inventory::{Product, ProductCreateInput, ProductId, Quantity, Sku, UnitOfMeasure};
    use domain_shared::{Currency, Money, TenantId};

    use super::*;
    use crate::reservation_port::{InMemoryReservationPort, StockSnapshot};

    fn sample_commerce() -> Commerce {
        Commerce::create(CreateCommerceInput {
            id: CommerceId::generate(),
            cnpj: Cnpj::parse("11222333000181").expect("cnpj"),
            legal_name: "Acme".into(),
            trade_name: None,
            tenant_id: TenantId::generate(),
        })
    }

    fn delivery_address(commerce: &Commerce) -> CommerceAddress {
        CommerceAddress::create(
            commerce,
            CreateCommerceAddressInput {
                id: CommerceAddressId::generate(),
                tenant_id: commerce.tenant_id(),
                commerce_id: commerce.id(),
                address_type: AddressType::Delivery,
                street: "Rua A".into(),
                number: "100".into(),
                district: None,
                city: "SP".into(),
                state: "SP".into(),
                postal_code: "01310100".into(),
                latitude: None,
                longitude: None,
                is_primary: true,
            },
            &[],
        )
        .expect("address")
    }

    fn sample_product(tenant_id: TenantId, price: i64) -> Product {
        Product::create(ProductCreateInput {
            id: ProductId::generate(),
            name: "Widget".into(),
            sku: Sku::parse("WGT-001").expect("sku"),
            unit_price: Money::new(price, Currency::brl()).expect("price"),
            tenant_id,
            active: true,
            category: None,
            unit_of_measure: UnitOfMeasure::Unit,
        })
    }

    fn draft_order_with_item() -> (Order, ProductId) {
        let commerce = sample_commerce();
        let tenant_id = commerce.tenant_id();
        let product = sample_product(tenant_id, 1_000);
        let product_id = product.id();
        let order = Order::create(OrderCreateInput {
            id: OrderId::generate(),
            tenant_id,
            commerce: commerce.clone(),
            created_by: UserId::generate(),
            source: OrderSource::SellerVisit,
            delivery_address: delivery_address(&commerce),
            notes: None,
        })
        .expect("create")
        .add_item(AddOrderItemInput {
            item_id: OrderItemId::generate(),
            product,
            quantity: Quantity::of(2).expect("qty"),
        })
        .expect("add item");
        (order, product_id)
    }

    #[test]
    fn given_draft_with_items_when_submit_then_pending_approval() {
        let order = draft_order_with_item().0.submit().expect("submit");
        assert_eq!(order.status(), OrderStatus::PendingApproval);
    }

    #[test]
    fn given_pending_approval_when_approve_then_approved() {
        let (order, product_id) = draft_order_with_item();
        let submitted = order.submit().expect("submit");
        let mut port = InMemoryReservationPort::new(StockSnapshot::default().with_balance(product_id, 100));
        let (approved, reservations) = submitted.approve(&mut port).expect("approve");
        assert_eq!(approved.status(), OrderStatus::Approved);
        assert_eq!(reservations.len(), 1);
    }
}
