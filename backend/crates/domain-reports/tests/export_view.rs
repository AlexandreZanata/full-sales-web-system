//! Report export view model and format renderers — Phase 42.

mod support;

use domain_reports::{
    ExportFormat, ExportMeta, ReportExportError, parse_export_view, render_export,
};
use uuid::Uuid;

use support::{assembly_with_sales, delivered_sale_fact, empty_report_canonical, fixed_driver_id};

#[test]
fn given_fixture_payload_when_parsed_then_row_count_and_totals_match() {
    let payload = assembly_with_sales(vec![delivered_sale_fact(
        domain_sales::DeclaredPaymentMethod::Pix,
        true,
    )])
    .assemble()
    .expect("assemble");

    let view = parse_export_view("DailyDriver", &payload.canonical_json).expect("parse");
    assert_eq!(view.sales.len(), 1);
    assert_eq!(view.driver_id, fixed_driver_id().as_uuid());
    assert_eq!(view.settlement.total_declared_cents, 45_000);
}

#[test]
fn given_empty_payload_when_parsed_then_zero_sales() {
    let view = parse_export_view("DailyDriver", &empty_report_canonical()).expect("parse");
    assert!(view.sales.is_empty());
    assert_eq!(view.settlement.total_declared_cents, 0);
}

#[test]
fn given_invalid_json_when_parsed_then_invalid_json() {
    let err = parse_export_view("DailyDriver", "{").expect_err("json");
    assert_eq!(err, ReportExportError::InvalidJson);
}

#[test]
fn given_commerce_period_or_consolidated_when_parsed_then_same_view_as_daily_driver() {
    let payload = assembly_with_sales(vec![]).assemble().expect("assemble");
    let daily = parse_export_view("DailyDriver", &payload.canonical_json).expect("daily");
    let commerce = parse_export_view("CommercePeriod", &payload.canonical_json).expect("commerce");
    let consolidated =
        parse_export_view("Consolidated", &payload.canonical_json).expect("consolidated");
    assert_eq!(commerce, daily);
    assert_eq!(consolidated, daily);
}

#[test]
fn given_daily_driver_csv_when_rendered_then_golden_header_and_driver_id() {
    let payload = assembly_with_sales(vec![delivered_sale_fact(
        domain_sales::DeclaredPaymentMethod::Pix,
        true,
    )])
    .assemble()
    .expect("assemble");
    let view = parse_export_view("DailyDriver", &payload.canonical_json).expect("parse");
    let meta = ExportMeta {
        report_id: Uuid::parse_str("0190e001-1111-2222-3333-444455556666").expect("id"),
        report_type: "DailyDriver".into(),
        period_start: view.period.start,
        verify_url: None,
    };

    let rendered = render_export(&view, &meta, ExportFormat::Csv, None).expect("csv");
    let text = String::from_utf8(rendered.bytes).expect("utf8");
    assert!(text.starts_with('\u{feff}'));
    assert!(text.contains("saleId,orderId,commerceId,amountCents,currency"));
    assert!(text.contains(&format!("driverId,{}", view.driver_id)));
    assert!(text.contains("salesCount,1"));
    assert_eq!(rendered.filename, "report-DailyDriver-2026-07-01.csv");
}

#[test]
fn given_missing_driver_id_when_parsed_then_missing_field() {
    let err = parse_export_view(
        "DailyDriver",
        r#"{"version":2,"period":{"start":"2026-07-01T00:00:00Z","end":"2026-07-07T23:59:59Z"},"sales":[],"declaredSettlement":{"totalDeclaredCents":0,"currency":"BRL","byPaymentMethod":{},"disclaimer":"x"}}"#,
    )
    .expect_err("missing");
    assert_eq!(err, ReportExportError::MissingField("driverId"));
}
