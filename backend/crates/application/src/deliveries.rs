use domain_deliveries::{ConfirmDeliveryInput, Delivery, DeliveryError};
use domain_identity::UserId;
use domain_media::FileId;
use domain_orders::{DeliveredItemInput, Order, OrderError};
use domain_sales::{Sale, SaleFromDeliveryInput, SaleId};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeliveriesAppError {
    #[error(transparent)]
    Delivery(#[from] DeliveryError),

    #[error(transparent)]
    Order(#[from] OrderError),

    #[error(transparent)]
    Sale(#[from] domain_sales::SaleError),
}

pub struct ConfirmDeliveryAndCreateSaleCommand {
    pub delivery: Delivery,
    pub order: Order,
    pub items: Vec<DeliveredItemInput>,
    pub proof_file_id: FileId,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub received_by_name: Option<String>,
    pub acting_driver: UserId,
    pub sale_id: SaleId,
}

pub struct ConfirmDeliveryAndCreateSaleResult {
    pub delivery: Delivery,
    pub order: Order,
    pub sale: Sale,
}

/// Single orchestration: confirm delivery, update order, build confirmed sale (Phase 13).
pub fn confirm_delivery_and_create_sale(
    command: ConfirmDeliveryAndCreateSaleCommand,
) -> Result<ConfirmDeliveryAndCreateSaleResult, DeliveriesAppError> {
    let delivery = command.delivery.confirm(
        ConfirmDeliveryInput {
            proof_file_id: Some(command.proof_file_id),
            latitude: command.latitude,
            longitude: command.longitude,
            received_by_name: command.received_by_name,
        },
        command.acting_driver,
    )?;

    let order = command.order.confirm_delivery(&command.items)?;
    let sale = Sale::from_delivery(SaleFromDeliveryInput {
        id: command.sale_id,
        driver_id: command.acting_driver,
        order: order.clone(),
    })?;

    Ok(ConfirmDeliveryAndCreateSaleResult {
        delivery,
        order,
        sale,
    })
}

pub struct SaleLineDto {
    pub product_id: uuid::Uuid,
    pub quantity: i32,
    pub unit_price_amount: i64,
    pub unit_price_currency: String,
    pub line_total_amount: i64,
}

pub fn sale_lines_from_result(sale: &Sale) -> Result<Vec<SaleLineDto>, domain_shared::DomainError> {
    Ok(sale
        .items()
        .iter()
        .map(|item| SaleLineDto {
            product_id: item.product_id().as_uuid(),
            quantity: item.quantity().value(),
            unit_price_amount: item.unit_price().amount_minor(),
            unit_price_currency: item.unit_price().currency().as_str().to_owned(),
            line_total_amount: item.line_total().amount_minor(),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use domain_commerces::{
        AddressType, Cnpj, Commerce, CommerceAddressId, CommerceId, CreateCommerceAddressInput,
        CreateCommerceInput,
    };
    use domain_deliveries::{Delivery, DeliveryCreateInput, DeliveryId};
    use domain_identity::UserId;
    use domain_inventory::{Product, ProductCreateInput, ProductId, Quantity, Sku, UnitOfMeasure};
    use domain_media::FileId;
    use domain_orders::{
        AddOrderItemInput, OrderCreateInput, OrderId, OrderItemId, OrderSource, OrderStatus,
    };
    use domain_sales::SaleStatus;
    use domain_shared::{Currency, Money, TenantId};

    use super::*;

    fn in_transit_fixtures() -> (Delivery, Order, UserId, DeliveredItemInput) {
        let driver = UserId::generate();
        let commerce = Commerce::create(CreateCommerceInput {
            id: CommerceId::generate(),
            cnpj: Cnpj::parse("11222333000181").expect("cnpj"),
            legal_name: "Acme".into(),
            trade_name: None,
            tenant_id: TenantId::generate(),
        });
        let tenant_id = commerce.tenant_id();
        let order_id = OrderId::generate();
        let address = domain_commerces::CommerceAddress::create(
            &commerce,
            CreateCommerceAddressInput {
                id: CommerceAddressId::generate(),
                tenant_id,
                commerce_id: commerce.id(),
                address_type: AddressType::Delivery,
                street: "Rua".into(),
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
        let product = Product::create(ProductCreateInput {
            id: ProductId::generate(),
            name: "Widget".into(),
            sku: Sku::parse("WGT-1").expect("sku"),
            unit_price: Money::new(2_000, Currency::brl()).expect("price"),
            tenant_id,
            active: true,
            category: None,
            unit_of_measure: UnitOfMeasure::Unit,
        });
        let item_id = OrderItemId::generate();
        let product_id = product.id();
        let order = Order::create(OrderCreateInput {
            id: order_id,
            tenant_id,
            commerce: commerce.clone(),
            created_by: UserId::generate(),
            source: OrderSource::SellerVisit,
            delivery_address: address,
            notes: None,
        })
        .expect("create")
        .add_item(AddOrderItemInput {
            item_id,
            product,
            quantity: Quantity::of(4).expect("qty"),
        })
        .expect("add")
        .submit()
        .expect("submit");

        let mut port = domain_orders::InMemoryReservationPort::new(
            domain_orders::StockSnapshot::default().with_balance(product_id, 100),
        );
        let order = order
            .approve(&mut port)
            .expect("approve")
            .0
            .start_picking()
            .expect("picking")
            .mark_in_transit()
            .expect("in transit");

        let delivery = Delivery::create(DeliveryCreateInput {
            id: DeliveryId::generate(),
            tenant_id,
            order_id,
            driver_id: driver,
        })
        .start_transit(driver)
        .expect("transit");

        let delivered = DeliveredItemInput {
            order_item_id: item_id,
            quantity_delivered: Quantity::of(4).expect("qty"),
        };

        (delivery, order, driver, delivered)
    }

    #[test]
    fn given_valid_confirm_when_orchestrate_then_sale_and_order_delivered() {
        let (delivery, order, driver, delivered) = in_transit_fixtures();
        let result = confirm_delivery_and_create_sale(ConfirmDeliveryAndCreateSaleCommand {
            delivery,
            order,
            items: vec![delivered],
            proof_file_id: FileId::generate(),
            latitude: Some(-23.5),
            longitude: Some(-46.6),
            received_by_name: Some("Joao".into()),
            acting_driver: driver,
            sale_id: SaleId::generate(),
        })
        .expect("confirm");

        assert_eq!(result.order.status(), OrderStatus::Delivered);
        assert_eq!(result.sale.status(), SaleStatus::Confirmed);
        assert_eq!(result.sale.items().len(), 1);
        assert!(result.sale.order_id().is_some());
        assert_eq!(result.sale.total().expect("total").amount_minor(), 8_000);
    }
}
