use rust_xlsxwriter::{Format, Workbook};

use crate::export::csv::export_filename;
use crate::export::error::ReportExportError;
use crate::export::format::{format_date_pt_br, format_money_brl};
use crate::export::view::{ExportMeta, ReportExportView, RenderedExport};

pub fn render_xlsx(view: &ReportExportView, meta: &ExportMeta) -> Result<RenderedExport, ReportExportError> {
    let mut workbook = Workbook::new();
    write_sales_sheet(&mut workbook, view).map_err(|_| ReportExportError::RenderFailed)?;
    write_summary_sheet(&mut workbook, view, meta).map_err(|_| ReportExportError::RenderFailed)?;

    let bytes = workbook.save_to_buffer().map_err(|_| ReportExportError::RenderFailed)?;
    Ok(RenderedExport {
        bytes,
        content_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        filename: export_filename(meta, "xlsx"),
    })
}

fn write_sales_sheet(workbook: &mut Workbook, view: &ReportExportView) -> Result<(), rust_xlsxwriter::XlsxError> {
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Sales")?;

    let header = Format::new().set_bold();
    worksheet.write_string_with_format(0, 0, "Sale ID", &header)?;
    worksheet.write_string_with_format(0, 1, "Order ID", &header)?;
    worksheet.write_string_with_format(0, 2, "Commerce ID", &header)?;
    worksheet.write_string_with_format(0, 3, "Amount (BRL)", &header)?;
    worksheet.write_string_with_format(0, 4, "Currency", &header)?;

    for (index, sale) in view.sales.iter().enumerate() {
        let row = (index + 1) as u32;
        worksheet.write_string(row, 0, sale.sale_id.to_string())?;
        if let Some(order_id) = sale.order_id {
            worksheet.write_string(row, 1, order_id.to_string())?;
        }
        worksheet.write_string(row, 2, sale.commerce_id.to_string())?;
        worksheet.write_string(row, 3, format_money_brl(sale.amount_cents, &sale.currency))?;
        worksheet.write_string(row, 4, &sale.currency)?;
    }

    let _ = worksheet;
    Ok(())
}

fn write_summary_sheet(
    workbook: &mut Workbook,
    view: &ReportExportView,
    meta: &ExportMeta,
) -> Result<(), rust_xlsxwriter::XlsxError> {
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Summary")?;

    worksheet.write_string(0, 0, "Report ID")?;
    worksheet.write_string(0, 1, meta.report_id.to_string())?;
    worksheet.write_string(1, 0, "Report type")?;
    worksheet.write_string(1, 1, &meta.report_type)?;
    worksheet.write_string(2, 0, "Driver ID")?;
    worksheet.write_string(2, 1, view.driver_id.to_string())?;
    worksheet.write_string(3, 0, "Period start")?;
    worksheet.write_string(3, 1, format_date_pt_br(view.period.start))?;
    worksheet.write_string(4, 0, "Period end")?;
    worksheet.write_string(4, 1, format_date_pt_br(view.period.end))?;
    worksheet.write_string(5, 0, "Sales count")?;
    worksheet.write_number(5, 1, view.sales.len() as f64)?;
    worksheet.write_string(6, 0, "Total declared")?;
    worksheet.write_string(
        6,
        1,
        format_money_brl(
            view.settlement.total_declared_cents,
            &view.settlement.currency,
        ),
    )?;

    let mut row = 8_u32;
    worksheet.write_string(row, 0, "By payment method")?;
    row += 1;
    for (method, amount) in &view.settlement.by_payment_method {
        worksheet.write_string(row, 0, method)?;
        worksheet.write_string(row, 1, format_money_brl(*amount, &view.settlement.currency))?;
        row += 1;
    }

    worksheet.write_string(row + 1, 0, "Disclaimer")?;
    worksheet.write_string(row + 2, 0, &view.settlement.disclaimer)?;

    let _ = worksheet;
    Ok(())
}
