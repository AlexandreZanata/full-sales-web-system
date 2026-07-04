use chrono::{DateTime, Utc};
use domain_commerces::Commerce;
use domain_identity::UserId;
use domain_inventory::{Product, Quantity};
use domain_orders::{Order, OrderId};
use domain_shared::{Money, TenantId};

use crate::audit_port::{PaymentDeclarationAuditEntry, PaymentDeclarationAuditPort};
use crate::declared_payment::DeclaredPayment;
use crate::declared_payment_method::DeclaredPaymentMethod;
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

pub struct SaleFromDeliveryInput {
    pub id: SaleId,
    pub driver_id: UserId,
    pub order: Order,
}

pub struct DeclarePaymentInput {
    pub method: DeclaredPaymentMethod,
    pub received: bool,
    pub declared_at: DateTime<Utc>,
    pub declaring_user: UserId,
    pub notes: Option<String>,
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
    order_id: Option<OrderId>,
    payment_method: PaymentMethod,
    declared_payment: DeclaredPayment,
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
            order_id: None,
            payment_method: input.payment_method,
            declared_payment: DeclaredPayment::not_declared(),
            tenant_id: input.tenant_id,
            status: SaleStatus::Pending,
            items: Vec::new(),
        })
    }

    /// Portal path — sale born from delivery with delivered quantities (Phase 13).
    pub fn from_delivery(input: SaleFromDeliveryInput) -> Result<Self, SaleError> {
        let order = input.order;
        let mut items = Vec::new();
        for line in order.items() {
            let quantity = line
                .quantity_delivered()
                .ok_or(SaleError::EmptySale)?;
            items.push(
                SaleItem::create(
                    line.product_id(),
                    quantity,
                    line.unit_price().clone(),
                )
                .map_err(|_| SaleError::EmptySale)?,
            );
        }
        if items.is_empty() {
            return Err(SaleError::EmptySale);
        }

        Ok(Self {
            id: input.id,
            driver_id: input.driver_id,
            commerce_id: order.commerce_id(),
            order_id: Some(order.id()),
            payment_method: PaymentMethod::NotDeclared,
            declared_payment: DeclaredPayment::not_declared(),
            tenant_id: order.tenant_id(),
            status: SaleStatus::Confirmed,
            items,
        })
    }

    /// RN-PAG2 — only the responsible driver may declare; RN-PAG3 — audit on change.
    pub fn declare_payment(
        mut self,
        input: DeclarePaymentInput,
        audit: &mut impl PaymentDeclarationAuditPort,
    ) -> Result<Self, SaleError> {
        if input.declaring_user != self.driver_id {
            return Err(SaleError::UnauthorizedPaymentDeclaration);
        }

        let previous = self.declared_payment.clone();
        let current = DeclaredPayment::apply(
            input.method,
            input.received,
            input.declared_at,
            input.declaring_user,
            input.notes,
        );
        audit.record_change(PaymentDeclarationAuditEntry {
            sale_id: self.id,
            actor_id: input.declaring_user,
            previous,
            current: current.clone(),
        })?;
        self.declared_payment = current;
        Ok(self)
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

    pub fn order_id(&self) -> Option<OrderId> {
        self.order_id
    }

    pub fn payment_method(&self) -> PaymentMethod {
        self.payment_method
    }

    pub fn declared_payment(&self) -> &DeclaredPayment {
        &self.declared_payment
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
        order_id: Option<OrderId>,
        payment_method: PaymentMethod,
        declared_payment: DeclaredPayment,
        tenant_id: TenantId,
        status: SaleStatus,
        items: Vec<SaleItem>,
    ) -> Self {
        Self {
            id,
            driver_id,
            commerce_id,
            order_id,
            payment_method,
            declared_payment,
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
