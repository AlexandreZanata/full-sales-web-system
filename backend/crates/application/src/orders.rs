use std::collections::HashMap;

use domain_inventory::{ProductId, StockReservation};
use domain_orders::{
    InMemoryReservationPort, Order, OrderError, OrderStatus, StockSnapshot,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrdersAppError {
    #[error(transparent)]
    Order(#[from] OrderError),

    #[error("order not found")]
    OrderNotFound,

    #[error("insufficient available stock for order approval")]
    InsufficientAvailableStock,
}

pub struct ApproveOrderResult {
    pub order: Order,
    pub reservations: Vec<StockReservation>,
}

/// Approves an order and builds reservation entities via the stock port (RN2).
pub fn approve_order(
    order: Order,
    balance_by_product: HashMap<ProductId, i32>,
    reserved_by_product: HashMap<ProductId, i32>,
) -> Result<ApproveOrderResult, OrdersAppError> {
    if order.status() != OrderStatus::PendingApproval {
        return Err(OrderError::InvalidTransition {
            from: order.status(),
            to: OrderStatus::Approved,
        }
        .into());
    }

    let mut port = InMemoryReservationPort::new(StockSnapshot::new(
        balance_by_product,
        reserved_by_product,
    ));
    let (approved, reservations) = order.approve(&mut port)?;
    Ok(ApproveOrderResult {
        order: approved,
        reservations,
    })
}

pub struct CancelOrderResult {
    pub order: Order,
    pub release_reservations: bool,
}

/// Cancels an order; caller releases reservations when `release_reservations` is true (RN6).
pub fn cancel_order(order: Order) -> Result<CancelOrderResult, OrdersAppError> {
    let previous = order.status();
    let cancelled = order.cancel()?;
    Ok(CancelOrderResult {
        order: cancelled,
        release_reservations: previous.had_active_reservations(),
    })
}

/// Maps domain reservations to infra insert lines.
pub fn reservation_lines(
    reservations: &[StockReservation],
) -> Vec<ReservationLineDto> {
    reservations
        .iter()
        .map(|r| ReservationLineDto {
            id: r.id().as_uuid(),
            order_id: r.order_id(),
            order_item_id: r.order_item_id(),
            product_id: r.product_id().as_uuid(),
            quantity: r.quantity().value(),
            driver_id: r.driver_id(),
        })
        .collect()
}

pub struct ReservationLineDto {
    pub id: uuid::Uuid,
    pub order_id: uuid::Uuid,
    pub order_item_id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub quantity: i32,
    pub driver_id: Option<uuid::Uuid>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use domain_commerces::{
        AddressType, Cnpj, Commerce, CommerceAddressId, CommerceId, CreateCommerceAddressInput,
        CreateCommerceInput,
    };
    use domain_identity::UserId;
    use domain_inventory::{Product, ProductCreateInput, ProductId, Quantity, Sku, UnitOfMeasure};
    use domain_orders::{
        AddOrderItemInput, OrderCreateInput, OrderId, OrderItemId, OrderSource, OrderStatus,
    };
    use domain_shared::{Currency, Money, TenantId};

    use super::*;

    fn submitted_order(product: Product) -> Order {
        let commerce = Commerce::create(CreateCommerceInput {
            id: CommerceId::generate(),
            cnpj: Cnpj::parse("11222333000181").expect("cnpj"),
            legal_name: "Acme".into(),
            trade_name: None,
            tenant_id: TenantId::generate(),
        });
        let address = domain_commerces::CommerceAddress::create(
            &commerce,
            CreateCommerceAddressInput {
                id: CommerceAddressId::generate(),
                tenant_id: commerce.tenant_id(),
                commerce_id: commerce.id(),
                address_type: AddressType::Delivery,
                street: "Rua A".into(),
                number: "1".into(),
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
        .expect("address");
        Order::create(OrderCreateInput {
            id: OrderId::generate(),
            tenant_id: commerce.tenant_id(),
            commerce,
            created_by: UserId::generate(),
            source: OrderSource::SellerVisit,
            delivery_address: address,
            notes: None,
        })
        .expect("create")
        .add_item(AddOrderItemInput {
            item_id: OrderItemId::generate(),
            product,
            quantity: Quantity::of(5).expect("qty"),
        })
        .expect("add")
        .submit()
        .expect("submit")
    }

    #[test]
    fn given_sufficient_stock_when_approve_order_then_approved() {
        let product = Product::create(ProductCreateInput {
            id: ProductId::generate(),
            name: "Item".into(),
            sku: Sku::parse("SKU-1").expect("sku"),
            unit_price: Money::new(500, Currency::brl()).expect("price"),
            tenant_id: TenantId::generate(),
            active: true,
            category: None,
            unit_of_measure: UnitOfMeasure::Unit,
        });
        let product_id = product.id();
        let order = submitted_order(product);
        let mut balances = HashMap::new();
        balances.insert(product_id, 20);
        let result = approve_order(order, balances, HashMap::new()).expect("approve");
        assert_eq!(result.order.status(), OrderStatus::Approved);
        assert_eq!(result.reservations.len(), 1);
    }

    #[test]
    fn given_approved_order_when_cancel_then_release_flag_set() {
        let product = Product::create(ProductCreateInput {
            id: ProductId::generate(),
            name: "Item".into(),
            sku: Sku::parse("SKU-2").expect("sku"),
            unit_price: Money::new(500, Currency::brl()).expect("price"),
            tenant_id: TenantId::generate(),
            active: true,
            category: None,
            unit_of_measure: UnitOfMeasure::Unit,
        });
        let product_id = product.id();
        let order = submitted_order(product);
        let mut balances = HashMap::new();
        balances.insert(product_id, 20);
        let approved = approve_order(order, balances, HashMap::new())
            .expect("approve")
            .order;
        let result = cancel_order(approved).expect("cancel");
        assert_eq!(result.order.status(), OrderStatus::Cancelled);
        assert!(result.release_reservations);
    }
}
