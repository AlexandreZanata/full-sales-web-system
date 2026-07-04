use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

use crate::export::error::ReportExportError;
use crate::export::view::{ExportPeriod, ExportSaleRow, ExportSettlement, ReportExportView};

pub fn parse_export_view(
    report_type: &str,
    canonical_json: &str,
) -> Result<ReportExportView, ReportExportError> {
    match report_type {
        "DailyDriver" => parse_daily_driver(canonical_json),
        "CommercePeriod" | "Consolidated" => {
            Err(ReportExportError::UnsupportedReportType(report_type.to_owned()))
        }
        other => Err(ReportExportError::UnsupportedReportType(other.to_owned())),
    }
}

fn parse_daily_driver(canonical_json: &str) -> Result<ReportExportView, ReportExportError> {
    let value: Value =
        serde_json::from_str(canonical_json).map_err(|_| ReportExportError::InvalidJson)?;

    let version = required_u64(&value, "version")? as u32;
    if version != 2 {
        return Err(ReportExportError::UnsupportedVersion(version));
    }

    let period = value
        .get("period")
        .ok_or(ReportExportError::MissingField("period"))?;
    let start = parse_rfc3339(
        required_str(period, "start")?,
        ReportExportError::MissingField("period.start"),
    )?;
    let end = parse_rfc3339(
        required_str(period, "end")?,
        ReportExportError::MissingField("period.end"),
    )?;

    let driver_id = required_uuid(&value, "driverId")?;
    let sales = parse_sales(value.get("sales").ok_or(ReportExportError::MissingField("sales"))?)?;
    let settlement = parse_settlement(
        value
            .get("declaredSettlement")
            .ok_or(ReportExportError::MissingField("declaredSettlement"))?,
    )?;

    Ok(ReportExportView {
        version,
        period: ExportPeriod { start, end },
        driver_id,
        sales,
        settlement,
    })
}

fn parse_sales(value: &Value) -> Result<Vec<ExportSaleRow>, ReportExportError> {
    let Some(items) = value.as_array() else {
        return Err(ReportExportError::MissingField("sales"));
    };

    items.iter().map(parse_sale_row).collect()
}

fn parse_sale_row(value: &Value) -> Result<ExportSaleRow, ReportExportError> {
    let sale_id = required_uuid(value, "saleId")?;
    let commerce_id = required_uuid(value, "commerceId")?;
    let amount_cents = required_i64(value, "amountCents")?;
    let currency = required_str(value, "currency")?.to_owned();
    let order_id = optional_uuid(value, "orderId")?;

    Ok(ExportSaleRow {
        sale_id,
        order_id,
        commerce_id,
        amount_cents,
        currency,
    })
}

fn parse_settlement(value: &Value) -> Result<ExportSettlement, ReportExportError> {
    let total_declared_cents = required_i64(value, "totalDeclaredCents")?;
    let currency = required_str(value, "currency")?.to_owned();
    let disclaimer = required_str(value, "disclaimer")?.to_owned();
    let by_payment_method = parse_payment_totals(
        value
            .get("byPaymentMethod")
            .ok_or(ReportExportError::MissingField("declaredSettlement.byPaymentMethod"))?,
    )?;

    Ok(ExportSettlement {
        total_declared_cents,
        currency,
        by_payment_method,
        disclaimer,
    })
}

fn parse_payment_totals(value: &Value) -> Result<Vec<(String, i64)>, ReportExportError> {
    let Some(map) = value.as_object() else {
        return Err(ReportExportError::MissingField(
            "declaredSettlement.byPaymentMethod",
        ));
    };

    let mut rows: Vec<(String, i64)> = map
        .iter()
        .map(|(method, amount)| {
            amount
                .as_i64()
                .ok_or(ReportExportError::MissingField(
                    "declaredSettlement.byPaymentMethod value",
                ))
                .map(|cents| (method.clone(), cents))
        })
        .collect::<Result<_, _>>()?;
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(rows)
}

fn required_str<'a>(value: &'a Value, field: &'static str) -> Result<&'a str, ReportExportError> {
    value
        .get(field)
        .and_then(|v| v.as_str())
        .ok_or(ReportExportError::MissingField(field))
}

fn required_u64(value: &Value, field: &'static str) -> Result<u64, ReportExportError> {
    value
        .get(field)
        .and_then(|v| v.as_u64())
        .ok_or(ReportExportError::MissingField(field))
}

fn required_i64(value: &Value, field: &'static str) -> Result<i64, ReportExportError> {
    value
        .get(field)
        .and_then(|v| v.as_i64())
        .ok_or(ReportExportError::MissingField(field))
}

fn required_uuid(value: &Value, field: &'static str) -> Result<Uuid, ReportExportError> {
    let raw = required_str(value, field)?;
    Uuid::parse_str(raw).map_err(|_| ReportExportError::MissingField(field))
}

fn optional_uuid(value: &Value, field: &'static str) -> Result<Option<Uuid>, ReportExportError> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(raw) => {
            let text = raw
                .as_str()
                .ok_or(ReportExportError::MissingField(field))?;
            Uuid::parse_str(text)
                .map(Some)
                .map_err(|_| ReportExportError::MissingField(field))
        }
    }
}

fn parse_rfc3339(raw: &str, err: ReportExportError) -> Result<DateTime<Utc>, ReportExportError> {
    DateTime::parse_from_rfc3339(raw)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|_| err)
}
