//! RN9, RN-PAG4, BR-RE-001 — report payload settlement expansion (Phase 15).

mod support;

use domain_reports::{
    is_eligible_for_report, sign_canonical_payload, verify_canonical_payload, PAYLOAD_VERSION,
    SETTLEMENT_DISCLAIMER, ReportError,
};
use domain_sales::{DeclaredPaymentMethod, SaleId};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use uuid::Uuid;

use support::{
    assembly_with_sales, delivered_sale_fact, fixed_driver_id, in_transit_sale_fact, sample_period,
};

// Contract: BR-RE-001 — deterministic canonical JSON for version 2 payload
#[test]
fn given_settled_sales_when_assembled_then_golden_canonical_json() {
    let payload = assembly_with_sales(vec![delivered_sale_fact(
        DeclaredPaymentMethod::Pix,
        true,
    )])
    .assemble()
    .expect("assemble");

    let expected = r#"{"declaredSettlement":{"byPaymentMethod":{"Pix":45000},"currency":"BRL","disclaimer":"Self-declared by seller. Not fiscal or bank proof.","totalDeclaredCents":45000},"driverId":"0190f1a2-b3c4-5678-9abc-def012345678","period":{"end":"2026-07-07T23:59:59Z","start":"2026-07-01T00:00:00Z"},"sales":[{"amountCents":45000,"commerceId":"0190f4d5-e6f7-8901-abcd-ef1234567892","currency":"BRL","orderId":"0190f3c4-d5e6-7890-abcd-ef1234567891","saleId":"0190f2b3-c4d5-6789-abcd-ef1234567890"}],"version":2}"#;

    assert_eq!(payload.canonical_json, expected);
    assert_eq!(PAYLOAD_VERSION, 2);
}

// Contract: RN-PAG4 — disclaimer states self-declared, non-fiscal proof
#[test]
fn given_declared_settlement_when_assembled_then_disclaimer_present() {
    let payload = assembly_with_sales(vec![delivered_sale_fact(
        DeclaredPaymentMethod::Cash,
        true,
    )])
    .assemble()
    .expect("assemble");

    assert!(payload.canonical_json.contains(SETTLEMENT_DISCLAIMER));
}

// Contract: RN-PAG1 — sale without declaration still produces valid report with zero settlement
#[test]
fn given_sale_without_declaration_when_assembled_then_settlement_zero() {
    let payload = assembly_with_sales(vec![delivered_sale_fact(
        DeclaredPaymentMethod::NotDeclared,
        false,
    )])
    .assemble()
    .expect("assemble");

    assert!(payload.canonical_json.contains(r#""totalDeclaredCents":0"#));
    assert!(payload.canonical_json.contains(r#""byPaymentMethod":{},"#));
    assert!(payload.canonical_json.contains(r#""sales":[{"#));
}

// Contract: RN9 — in-flight orders excluded from report sections
#[test]
fn given_in_transit_order_when_assembled_then_sale_excluded() {
    let payload = assembly_with_sales(vec![in_transit_sale_fact()])
        .assemble()
        .expect("assemble");

    assert_eq!(payload.canonical_json, empty_report_canonical());
    assert!(!is_eligible_for_report(&in_transit_sale_fact()));
}

// Contract: RN9 — delivered order included
#[test]
fn given_delivered_order_when_eligibility_checked_then_included() {
    assert!(is_eligible_for_report(&delivered_sale_fact(
        DeclaredPaymentMethod::Pix,
        true,
    )));
}

// Contract: BR-RE-001/002 — signature verifies on version 2 payload; tamper fails
#[test]
fn given_version_two_payload_when_signed_then_verify_passes_until_tampered() {
    let payload = assembly_with_sales(vec![delivered_sale_fact(
        DeclaredPaymentMethod::Pix,
        true,
    )])
    .assemble()
    .expect("assemble");

    let signing_key = SigningKey::generate(&mut OsRng);
    let signature = sign_canonical_payload(&payload.canonical_json, &signing_key);
    assert!(verify_canonical_payload(
        &payload.canonical_json,
        &signature,
        &signing_key.verifying_key()
    ));

    let tampered = payload.canonical_json.replace("45000", "45001");
    assert!(!verify_canonical_payload(
        &tampered,
        &signature,
        &signing_key.verifying_key()
    ));
}

// Contract: multiple payment methods aggregate by method
#[test]
fn given_mixed_declarations_when_assembled_then_totals_by_method() {
    let mut pix_sale = delivered_sale_fact(DeclaredPaymentMethod::Pix, true);
    pix_sale.sale_id = SaleId::from_uuid(
        Uuid::parse_str("0190c001-1111-2222-3333-444455556666").expect("sale"),
    );
    pix_sale.amount_cents = 30_000;

    let mut cash_sale = delivered_sale_fact(DeclaredPaymentMethod::Cash, true);
    cash_sale.sale_id = SaleId::from_uuid(
        Uuid::parse_str("0190c002-2222-3333-4444-555566667777").expect("sale"),
    );
    cash_sale.amount_cents = 15_000;

    let payload = assembly_with_sales(vec![cash_sale, pix_sale])
        .assemble()
        .expect("assemble");

    assert!(payload.canonical_json.contains(r#""Cash":15000"#));
    assert!(payload.canonical_json.contains(r#""Pix":30000"#));
    assert!(payload.canonical_json.contains(r#""totalDeclaredCents":45000"#));
}

// Contract: invalid period rejected
#[test]
fn given_end_before_start_when_assembled_then_invalid_period() {
    let mut input = assembly_with_sales(vec![]);
    input.period.end = input.period.start - chrono::Duration::seconds(1);
    let err = input.assemble().expect_err("period");
    assert_eq!(err, ReportError::InvalidPeriod);
}

#[test]
fn given_empty_period_sales_when_assembled_then_zero_report() {
    let payload = domain_reports::ReportAssemblyInput {
        period: sample_period(),
        driver_id: fixed_driver_id(),
        sales: vec![],
    }
    .assemble()
    .expect("assemble");

    assert_eq!(payload.canonical_json, empty_report_canonical());
}

fn empty_report_canonical() -> String {
    r#"{"declaredSettlement":{"byPaymentMethod":{},"currency":"BRL","disclaimer":"Self-declared by seller. Not fiscal or bank proof.","totalDeclaredCents":0},"driverId":"0190f1a2-b3c4-5678-9abc-def012345678","period":{"end":"2026-07-07T23:59:59Z","start":"2026-07-01T00:00:00Z"},"sales":[],"version":2}"#.to_string()
}
