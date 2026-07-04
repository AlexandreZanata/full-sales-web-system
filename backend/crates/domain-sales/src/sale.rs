use domain_commerces::Commerce;
use domain_identity::UserId;
use domain_inventory::{Product, Quantity};
use domain_shared::{Money, TenantId};

use crate::error::SaleError;
use crate::payment_method::PaymentMethod;
use crate::sale_id::SaleId;
use crate::sale_item::SaleItem;
use crate::sale_status::SaleStatus;

pub struct SaleCreateInput {
    pub id: SaleId,
    pub driver_id: UserId,
    pub commerce: Commerce,
    pub payment_method: PaymentMethod,
    pub tenant_id: TenantId,
}

pub struct AddSaleItemInput {
    pub product: Product,
    pub quantity: Quantity,
}

/// Commercial transaction aggregate — Pending | Confirmed | Cancelled.
#[derive(Debug, Clone)]
pub struct Sale {
    id: SaleId,
    driver_id: UserId,
    commerce_id: domain_commerces::CommerceId,
    payment_method: PaymentMethod,
    tenant_id: TenantId,
    status: SaleStatus,
    items: Vec<SaleItem>,
}

impl Sale {
    pub fn create(input: SaleCreateInput) -> Result<Self, SaleError> {
        if !input.commerce.is_active() {
            return Err(SaleError::InactiveCommerce);
        }
        Ok(Self {
            id: input.id,
            driver_id: input.driver_id,
            commerce_id: input.commerce.id(),
            payment_method: input.payment_method,
            tenant_id: input.tenant_id,
            status: SaleStatus::Pending,
            items: Vec::new(),
        })
    }

    pub fn id(&self) -> SaleId {
        self.id
    }

    pub fn driver_id(&self) -> UserId {
        self.driver_id
    }

    pub fn commerce_id(&self) -> domain_commerces::CommerceId {
        self.commerce_id
    }

    pub fn payment_method(&self) -> PaymentMethod {
        self.payment_method
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn status(&self) -> SaleStatus {
        self.status
    }

    pub fn items(&self) -> &[SaleItem] {
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

    pub fn add_item(mut self, input: AddSaleItemInput) -> Result<Self, SaleError> {
        self.assert_can_transition(SaleStatus::Pending)?;
        if !input.product.is_active() {
            return Err(SaleError::InactiveProduct);
        }
        let item = SaleItem::create(
            input.product.id(),
            input.quantity,
            input.product.unit_price().clone(),
        )
        .map_err(|_| SaleError::EmptySale)?;
        self.items.push(item);
        Ok(self)
    }

    pub fn confirm(mut self) -> Result<Self, SaleError> {
        self.assert_can_transition(SaleStatus::Confirmed)?;
        if self.items.is_empty() {
            return Err(SaleError::EmptySale);
        }
        self.status = SaleStatus::Confirmed;
        Ok(self)
    }

    pub fn cancel(mut self) -> Result<Self, SaleError> {
        self.assert_can_transition(SaleStatus::Cancelled)?;
        self.status = SaleStatus::Cancelled;
        Ok(self)
    }

    pub fn restore(
        id: SaleId,
        driver_id: UserId,
        commerce_id: domain_commerces::CommerceId,
        payment_method: PaymentMethod,
        tenant_id: TenantId,
        status: SaleStatus,
        items: Vec<SaleItem>,
    ) -> Self {
        Self {
            id,
            driver_id,
            commerce_id,
            payment_method,
            tenant_id,
            status,
            items,
        }
    }

    fn assert_can_transition(&self, target: SaleStatus) -> Result<(), SaleError> {
        if self.status != SaleStatus::Pending {
            return Err(SaleError::InvalidTransition {
                from: self.status,
                to: target,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use domain_commerces::{Cnpj, Commerce, CommerceId, CreateCommerceInput};
    use domain_identity::UserId;
    use domain_inventory::{Product, ProductCreateInput, ProductId, Quantity, Sku, UnitOfMeasure};
    use domain_shared::{Currency, Money, TenantId};

    use super::*;

    fn sample_commerce() -> Commerce {
        Commerce::create(CreateCommerceInput {
            id: CommerceId::generate(),
            cnpj: Cnpj::parse("11222333000181").expect("cnpj"),
            legal_name: "Acme".into(),
            trade_name: None,
            tenant_id: TenantId::generate(),
        })
    }

    fn sample_product(tenant_id: TenantId) -> Product {
        Product::create(ProductCreateInput {
            id: ProductId::generate(),
            name: "Widget".into(),
            sku: Sku::parse("WGT-001").expect("sku"),
            unit_price: Money::new(1_000, Currency::brl()).expect("price"),
            tenant_id,
            active: true,
            category: None,
            unit_of_measure: UnitOfMeasure::Unit,
        })
    }

    // Contract: STATE-MACHINES — Pending → Confirmed
    #[test]
    fn given_pending_sale_with_items_when_confirm_then_confirmed() {
        let commerce = sample_commerce();
        let tenant_id = commerce.tenant_id();
        let product = sample_product(tenant_id);
        let sale = Sale::create(SaleCreateInput {
            id: SaleId::generate(),
            driver_id: UserId::generate(),
            commerce,
            payment_method: PaymentMethod::Cash,
            tenant_id,
        })
        .expect("create")
        .add_item(AddSaleItemInput {
            product,
            quantity: Quantity::of(2).expect("qty"),
        })
        .expect("add item")
        .confirm()
        .expect("confirm");
        assert_eq!(sale.status(), SaleStatus::Confirmed);
    }

    // Contract: STATE-MACHINES — Confirmed → Pending fails
    #[test]
    fn given_confirmed_sale_when_confirm_again_then_invalid_transition() {
        let commerce = sample_commerce();
        let tenant_id = commerce.tenant_id();
        let product = sample_product(tenant_id);
        let sale = Sale::create(SaleCreateInput {
            id: SaleId::generate(),
            driver_id: UserId::generate(),
            commerce,
            payment_method: PaymentMethod::Pix,
            tenant_id,
        })
        .expect("create")
        .add_item(AddSaleItemInput {
            product,
            quantity: Quantity::of(1).expect("qty"),
        })
        .expect("add")
        .confirm()
        .expect("confirm");
        let err = sale.confirm().expect_err("must fail");
        assert!(matches!(
            err,
            SaleError::InvalidTransition {
                from: SaleStatus::Confirmed,
                to: SaleStatus::Confirmed
            }
        ));
    }

    // Contract: STATE-MACHINES — Pending → Cancelled (UC-001 AF-2)
    #[test]
    fn given_pending_sale_when_cancel_then_cancelled() {
        let commerce = sample_commerce();
        let tenant_id = commerce.tenant_id();
        let product = sample_product(tenant_id);
        let sale = Sale::create(SaleCreateInput {
            id: SaleId::generate(),
            driver_id: UserId::generate(),
            commerce,
            payment_method: PaymentMethod::Cash,
            tenant_id,
        })
        .expect("create")
        .add_item(AddSaleItemInput {
            product,
            quantity: Quantity::of(1).expect("qty"),
        })
        .expect("add")
        .cancel()
        .expect("cancel");
        assert_eq!(sale.status(), SaleStatus::Cancelled);
    }

    // Contract: BR-SA-003 — Confirmed sale cannot be cancelled
    #[test]
    fn given_confirmed_sale_when_cancel_then_invalid_transition() {
        let commerce = sample_commerce();
        let tenant_id = commerce.tenant_id();
        let product = sample_product(tenant_id);
        let sale = Sale::create(SaleCreateInput {
            id: SaleId::generate(),
            driver_id: UserId::generate(),
            commerce,
            payment_method: PaymentMethod::Debit,
            tenant_id,
        })
        .expect("create")
        .add_item(AddSaleItemInput {
            product,
            quantity: Quantity::of(1).expect("qty"),
        })
        .expect("add")
        .confirm()
        .expect("confirm");
        let err = sale.cancel().expect_err("must fail");
        assert!(matches!(
            err,
            SaleError::InvalidTransition {
                from: SaleStatus::Confirmed,
                to: SaleStatus::Cancelled
            }
        ));
    }
}
