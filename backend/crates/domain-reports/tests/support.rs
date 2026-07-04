//! Shared fixtures for domain-reports contract tests.

use chrono::{TimeZone, Utc};
use domain_commerces::CommerceId;
use domain_identity::UserId;
use domain_orders::{OrderId, OrderStatus};
use domain_reports::{ReportAssemblyInput, ReportPeriod, ReportSaleFact};
use domain_sales::{DeclaredPaymentMethod, SaleId, SaleStatus};
use uuid::Uuid;

pub fn fixed_driver_id() -> UserId {
    UserId::from_uuid(Uuid::parse_str("0190f1a2-b3c4-5678-9abc-def012345678").expect("driver"))
}

pub fn fixed_sale_id() -> SaleId {
    SaleId::from_uuid(Uuid::parse_str("0190f2b3-c4d5-6789-abcd-ef1234567890").expect("sale"))
}

pub fn fixed_order_id() -> OrderId {
    OrderId::from_uuid(Uuid::parse_str("0190f3c4-d5e6-7890-abcd-ef1234567891").expect("order"))
}

pub fn fixed_commerce_id() -> CommerceId {
    CommerceId::from_uuid(
        Uuid::parse_str("0190f4d5-e6f7-8901-abcd-ef1234567892").expect("commerce"),
    )
}

pub fn sample_period() -> ReportPeriod {
    ReportPeriod {
        start: Utc.with_ymd_and_hms(2026, 7, 1, 0, 0, 0).unwrap(),
        end: Utc.with_ymd_and_hms(2026, 7, 7, 23, 59, 59).unwrap(),
    }
}

pub fn delivered_sale_fact(
    declared_method: DeclaredPaymentMethod,
    declared_received: bool,
) -> ReportSaleFact {
    ReportSaleFact {
        sale_id: fixed_sale_id(),
        order_id: Some(fixed_order_id()),
        commerce_id: fixed_commerce_id(),
        amount_cents: 45_000,
        currency: "BRL".into(),
        sale_status: SaleStatus::Confirmed,
        order_status: Some(OrderStatus::Delivered),
        declared_method,
        declared_received,
    }
}

pub fn in_transit_sale_fact() -> ReportSaleFact {
    ReportSaleFact {
        sale_id: SaleId::from_uuid(Uuid::parse_str("0190aaaa-bbbb-cccc-dddd-ef1234567893").expect("sale")),
        order_id: Some(OrderId::from_uuid(
            Uuid::parse_str("0190bbbb-cccc-dddd-eeee-ef1234567894").expect("order"),
        )),
        commerce_id: fixed_commerce_id(),
        amount_cents: 10_000,
        currency: "BRL".into(),
        sale_status: SaleStatus::Confirmed,
        order_status: Some(OrderStatus::InTransit),
        declared_method: DeclaredPaymentMethod::Pix,
        declared_received: true,
    }
}

pub fn assembly_with_sales(sales: Vec<ReportSaleFact>) -> ReportAssemblyInput {
    ReportAssemblyInput {
        period: sample_period(),
        driver_id: fixed_driver_id(),
        sales,
    }
}
