use std::collections::HashMap;

use domain_commerces::{Commerce, CommerceAddress};
use domain_identity::UserId;
use domain_inventory::{Product, ProductId, StockReservation};
use domain_orders::{
    AddOrderItemInput, InMemoryReservationPort, Order, OrderError, OrderId, OrderItemId,
    OrderSource, OrderStatus, StockSnapshot,
};
use domain_shared::TenantId;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum OrdersAppError {
    #[error(transparent)]
    Order(#[from] OrderError),

    #[error(transparent)]
    Inventory(#[from] domain_inventory::InventoryError),

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
    if order.status() != OrderStatus::PendingApproval && order.status() != OrderStatus::Paid {
        return Err(OrderError::InvalidTransition {
            from: order.status(),
            to: OrderStatus::Approved,
        }
        .into());
    }

    let mut port =
        InMemoryReservationPort::new(StockSnapshot::new(balance_by_product, reserved_by_product));
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
pub fn reservation_lines(reservations: &[StockReservation]) -> Vec<ReservationLineDto> {
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

pub struct PortalOrderLineInput {
    pub item_id: OrderItemId,
    pub product: Product,
    pub quantity: i32,
}

pub struct CreatePortalOrderCommand {
    pub order_id: OrderId,
    pub tenant_id: TenantId,
    pub commerce: Commerce,
    pub delivery_address: CommerceAddress,
    pub created_by: UserId,
    pub notes: Option<String>,
    pub lines: Vec<PortalOrderLineInput>,
}

pub struct UpdatePortalDraftCommand {
    pub order: Order,
    pub commerce: Commerce,
    pub delivery_address: CommerceAddress,
    pub notes: Option<String>,
    pub lines: Vec<PortalOrderLineInput>,
}

pub fn create_portal_order(command: CreatePortalOrderCommand) -> Result<Order, OrdersAppError> {
    let mut order = Order::create(domain_orders::OrderCreateInput {
        id: command.order_id,
        tenant_id: command.tenant_id,
        commerce: command.commerce,
        created_by: command.created_by,
        source: OrderSource::CommercePortal,
        delivery_address: command.delivery_address,
        notes: command.notes,
    })?;
    for line in command.lines {
        let quantity = domain_inventory::Quantity::of(line.quantity)?;
        order = order.add_item(AddOrderItemInput {
            item_id: line.item_id,
            product: line.product,
            quantity,
        })?;
    }
    Ok(order)
}

pub fn update_portal_draft(command: UpdatePortalDraftCommand) -> Result<Order, OrdersAppError> {
    let lines = command
        .lines
        .into_iter()
        .map(|line| {
            Ok(AddOrderItemInput {
                item_id: line.item_id,
                product: line.product,
                quantity: domain_inventory::Quantity::of(line.quantity)?,
            })
        })
        .collect::<Result<Vec<_>, domain_inventory::InventoryError>>()?;
    command
        .order
        .update_draft(
            &command.commerce,
            command.delivery_address,
            command.notes,
            lines,
        )
        .map_err(OrdersAppError::from)
}

pub fn submit_portal_order(order: Order) -> Result<Order, OrdersAppError> {
    order.submit().map_err(OrdersAppError::from)
}

pub fn submit_portal_order_online(order: Order) -> Result<Order, OrdersAppError> {
    order
        .submit_for_online_payment()
        .map_err(OrdersAppError::from)
}

pub fn reject_order(order: Order, reason: &str) -> Result<Order, OrdersAppError> {
    order.reject(reason).map_err(OrdersAppError::from)
}

pub struct OrderDto {
    pub id: Uuid,
    pub commerce_id: Uuid,
    pub status: OrderStatus,
    pub delivery_address_id: Uuid,
    pub notes: Option<String>,
    pub total_amount: i64,
    pub total_currency: String,
    pub rejection_reason: Option<String>,
    pub items: Vec<OrderItemDto>,
}

pub struct OrderItemDto {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price_amount: i64,
    pub unit_price_currency: String,
    pub line_total_amount: i64,
}

pub fn order_to_dto(order: &Order) -> Result<OrderDto, domain_shared::DomainError> {
    let total = order.total()?;
    Ok(OrderDto {
        id: order.id().as_uuid(),
        commerce_id: order.commerce_id().as_uuid(),
        status: order.status(),
        delivery_address_id: order.delivery_address_id().as_uuid(),
        notes: order.notes().map(str::to_owned),
        total_amount: total.amount_minor(),
        total_currency: total.currency().as_str().to_owned(),
        rejection_reason: order.rejection_reason().map(str::to_owned),
        items: order
            .items()
            .iter()
            .map(|item| OrderItemDto {
                id: item.id().as_uuid(),
                product_id: item.product_id().as_uuid(),
                quantity: item.quantity_requested().value(),
                unit_price_amount: item.unit_price().amount_minor(),
                unit_price_currency: item.unit_price().currency().as_str().to_owned(),
                line_total_amount: item.line_total().amount_minor(),
            })
            .collect(),
    })
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
