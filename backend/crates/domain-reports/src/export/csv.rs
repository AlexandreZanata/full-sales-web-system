use crate::export::error::ReportExportError;
use crate::export::format::{escape_csv_field, period_start_filename};
use crate::export::view::{ExportMeta, RenderedExport, ReportExportView};

pub fn render_csv(
    view: &ReportExportView,
    meta: &ExportMeta,
) -> Result<RenderedExport, ReportExportError> {
    let mut output = String::from("\u{feff}");
    output.push_str("field,value\n");
    output.push_str("driverId,");
    output.push_str(&view.driver_id.to_string());
    output.push('\n');
    output.push_str("salesCount,");
    output.push_str(&view.sales.len().to_string());
    output.push('\n');
    output.push_str("totalDeclaredCents,");
    output.push_str(&view.settlement.total_declared_cents.to_string());
    output.push('\n');
    output.push('\n');
    output.push_str("saleId,orderId,commerceId,amountCents,currency\n");

    for sale in &view.sales {
        output.push_str(&escape_csv_field(&sale.sale_id.to_string()));
        output.push(',');
        output.push_str(&escape_csv_field(
            &sale.order_id.map(|id| id.to_string()).unwrap_or_default(),
        ));
        output.push(',');
        output.push_str(&escape_csv_field(&sale.commerce_id.to_string()));
        output.push(',');
        output.push_str(&sale.amount_cents.to_string());
        output.push(',');
        output.push_str(&escape_csv_field(&sale.currency));
        output.push('\n');
    }

    let filename = export_filename(meta, "csv");
    Ok(RenderedExport {
        bytes: output.into_bytes(),
        content_type: "text/csv; charset=utf-8",
        filename,
    })
}

pub fn export_filename(meta: &ExportMeta, extension: &str) -> String {
    format!(
        "report-{}-{}.{}",
        meta.report_type,
        period_start_filename(meta.period_start),
        extension
    )
}
