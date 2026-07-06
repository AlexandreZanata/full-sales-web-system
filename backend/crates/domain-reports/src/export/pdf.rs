use std::io::{BufWriter, Cursor};

use printpdf::{BuiltinFont, Mm, PdfDocument, PdfLayerReference};

use crate::export::csv::export_filename;
use crate::export::error::ReportExportError;
use crate::export::format::{format_date_pt_br, format_money_brl};
use crate::export::view::{
    ExportBranding, ExportMeta, PDF_ROWS_PER_PAGE, RenderedExport, ReportExportView,
};

const PAGE_WIDTH: f32 = 210.0;
const PAGE_HEIGHT: f32 = 297.0;
const LEFT: f32 = 18.0;
const TOP: f32 = 280.0;
const LINE: f32 = 5.5;

pub fn render_pdf(
    view: &ReportExportView,
    meta: &ExportMeta,
    branding: &ExportBranding,
) -> Result<RenderedExport, ReportExportError> {
    let (doc, page1, layer1) =
        PdfDocument::new("Report export", Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|_| ReportExportError::RenderFailed)?;
    let font_bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|_| ReportExportError::RenderFailed)?;

    let mut layer = doc.get_page(page1).get_layer(layer1);
    let mut y = TOP;

    y = write_line(&layer, &font_bold, 14.0, LEFT, y, &branding.display_name);
    y = write_line(
        &layer,
        &font,
        10.0,
        LEFT,
        y,
        &format!("Report type: {}", meta.report_type),
    );
    y = write_line(
        &layer,
        &font,
        10.0,
        LEFT,
        y,
        &format!(
            "Period: {} – {}",
            format_date_pt_br(view.period.start),
            format_date_pt_br(view.period.end)
        ),
    );
    y = write_line(
        &layer,
        &font,
        10.0,
        LEFT,
        y,
        &format!("Driver ID: {}", view.driver_id),
    );
    y -= LINE;

    y = write_line(&layer, &font_bold, 11.0, LEFT, y, "Sales");
    y = write_line(
        &layer,
        &font,
        9.0,
        LEFT,
        y,
        "Sale ID | Order ID | Commerce ID | Amount",
    );

    let mut page_index = 0_usize;
    for (index, sale) in view.sales.iter().enumerate() {
        if index > 0 && index % PDF_ROWS_PER_PAGE == 0 {
            let (page, layer_ref) = doc.add_page(Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");
            layer = doc.get_page(page).get_layer(layer_ref);
            y = TOP;
            page_index += 1;
            y = write_line(
                &layer,
                &font_bold,
                10.0,
                LEFT,
                y,
                &format!("Sales (continued — page {})", page_index + 1),
            );
        }

        let order = sale
            .order_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "-".to_owned());
        let line = format!(
            "{} | {} | {} | {}",
            sale.sale_id,
            order,
            sale.commerce_id,
            format_money_brl(sale.amount_cents, &sale.currency)
        );
        y = write_line(&layer, &font, 8.5, LEFT, y, &line);
    }

    y -= LINE;
    y = write_line(&layer, &font_bold, 11.0, LEFT, y, "Declared settlement");
    y = write_line(
        &layer,
        &font,
        10.0,
        LEFT,
        y,
        &format!(
            "Total declared: {}",
            format_money_brl(
                view.settlement.total_declared_cents,
                &view.settlement.currency
            )
        ),
    );
    for (method, amount) in &view.settlement.by_payment_method {
        y = write_line(
            &layer,
            &font,
            9.5,
            LEFT,
            y,
            &format!(
                "{method}: {}",
                format_money_brl(*amount, &view.settlement.currency)
            ),
        );
    }
    write_line(&layer, &font, 8.5, LEFT, y, &view.settlement.disclaimer);

    let footer_y = PAGE_HEIGHT - 20.0;
    write_line(
        &layer,
        &font,
        8.0,
        LEFT,
        footer_y,
        &format!("Report ID: {}", meta.report_id),
    );
    if let Some(url) = &meta.verify_url {
        write_line(
            &layer,
            &font,
            8.0,
            LEFT,
            footer_y - LINE,
            &format!("Verify: {url}"),
        );
    }

    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    doc.save(&mut buffer)
        .map_err(|_| ReportExportError::RenderFailed)?;
    let bytes = buffer
        .into_inner()
        .map_err(|_| ReportExportError::RenderFailed)?
        .into_inner();

    Ok(RenderedExport {
        bytes,
        content_type: "application/pdf",
        filename: export_filename(meta, "pdf"),
    })
}

fn write_line(
    layer: &PdfLayerReference,
    font: &printpdf::IndirectFontRef,
    size: f32,
    x: f32,
    y: f32,
    text: &str,
) -> f32 {
    layer.use_text(text, size, Mm(x), Mm(y), font);
    y - LINE
}
