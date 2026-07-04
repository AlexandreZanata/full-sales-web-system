use chrono::{DateTime, Utc};
use domain_commerces::CommerceId;
use domain_identity::UserId;
use domain_orders::OrderId;
use domain_orders::OrderStatus;
use domain_sales::SaleId;
use domain_sales::{DeclaredPaymentMethod, SaleStatus};
use serde_json::{Value, json};

use crate::canonical::to_canonical_json;
use crate::error::ReportError;

pub const PAYLOAD_VERSION: u32 = 2;
pub const SETTLEMENT_DISCLAIMER: &str = "Self-declared by seller. Not fiscal or bank proof.";

#[derive(Debug, Clone)]
pub struct ReportPeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ReportSaleFact {
    pub sale_id: SaleId,
    pub order_id: Option<OrderId>,
    pub commerce_id: CommerceId,
    pub amount_cents: i64,
    pub currency: String,
    pub sale_status: SaleStatus,
    pub order_status: Option<OrderStatus>,
    pub declared_method: DeclaredPaymentMethod,
    pub declared_received: bool,
}

#[derive(Debug, Clone)]
pub struct ReportAssemblyInput {
    pub period: ReportPeriod,
    pub driver_id: UserId,
    pub sales: Vec<ReportSaleFact>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssembledReportPayload {
    pub canonical_json: String,
}

impl ReportAssemblyInput {
    pub fn assemble(self) -> Result<AssembledReportPayload, ReportError> {
        if self.period.end < self.period.start {
            return Err(ReportError::InvalidPeriod);
        }

        let mut included: Vec<&ReportSaleFact> = self
            .sales
            .iter()
            .filter(|sale| is_eligible_for_report(sale))
            .collect();
        included.sort_by_key(|sale| sale.sale_id.as_uuid());

        for sale in &included {
            if sale.amount_cents < 0 {
                return Err(ReportError::NegativeSaleAmount);
            }
        }

        let currency = included
            .first()
            .map(|sale| sale.currency.as_str())
            .unwrap_or("BRL");

        let sales_json: Vec<Value> = included.iter().map(|sale| sale_to_json(sale)).collect();

        let settlement = build_declared_settlement(&included, currency);

        let payload = json!({
            "version": PAYLOAD_VERSION,
            "period": {
                "start": format_rfc3339(self.period.start),
                "end": format_rfc3339(self.period.end),
            },
            "driverId": self.driver_id.as_uuid(),
            "sales": sales_json,
            "declaredSettlement": settlement,
        });

        Ok(AssembledReportPayload {
            canonical_json: to_canonical_json(&payload),
        })
    }
}

/// RN9 — include delivered orders and confirmed field sales only.
pub fn is_eligible_for_report(sale: &ReportSaleFact) -> bool {
    if sale.sale_status != SaleStatus::Confirmed {
        return false;
    }
    match sale.order_status {
        None => true,
        Some(OrderStatus::Delivered | OrderStatus::PartiallyDelivered) => true,
        Some(_) => false,
    }
}

fn sale_to_json(sale: &ReportSaleFact) -> Value {
    let mut row = json!({
        "saleId": sale.sale_id.as_uuid(),
        "commerceId": sale.commerce_id.as_uuid(),
        "amountCents": sale.amount_cents,
        "currency": sale.currency,
    });
    if let Some(order_id) = sale.order_id {
        row["orderId"] = json!(order_id.as_uuid());
    }
    row
}

fn build_declared_settlement(sales: &[&ReportSaleFact], currency: &str) -> Value {
    let mut totals: Vec<(DeclaredPaymentMethod, i64)> = Vec::new();

    for sale in sales {
        if !sale.declared_received || sale.declared_method == DeclaredPaymentMethod::NotDeclared {
            continue;
        }
        if let Some(entry) = totals
            .iter_mut()
            .find(|(method, _)| *method == sale.declared_method)
        {
            entry.1 = entry.1.saturating_add(sale.amount_cents);
        } else {
            totals.push((sale.declared_method, sale.amount_cents));
        }
    }

    totals.sort_by_key(|(method, _)| method.as_str());

    let mut by_method = serde_json::Map::new();
    let mut total_declared = 0_i64;
    for (method, amount) in totals {
        total_declared = total_declared.saturating_add(amount);
        by_method.insert(method.as_str().to_string(), json!(amount));
    }

    json!({
        "totalDeclaredCents": total_declared,
        "currency": currency,
        "byPaymentMethod": Value::Object(by_method),
        "disclaimer": SETTLEMENT_DISCLAIMER,
    })
}

fn format_rfc3339(value: DateTime<Utc>) -> String {
    value.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}
