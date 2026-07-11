//! Official-style PDF export — dog-saru visual language via printpdf.

use std::io::{BufWriter, Cursor};

use printpdf::{BuiltinFont, Mm, PdfDocument};

use crate::export::csv::export_filename;
use crate::export::error::ReportExportError;
use crate::export::pdf_body::{draw_footer, draw_sales_heading, draw_settlement, draw_table_row};
use crate::export::pdf_draw::{self, TOP_Y};
use crate::export::pdf_header::{draw_header, draw_kpis, draw_meta};
use crate::export::view::{
    ExportBranding, ExportMeta, PDF_ROWS_PER_PAGE, RenderedExport, ReportExportView,
};

const PAGE_W: f32 = pdf_draw::PAGE_W;
const PAGE_H: f32 = pdf_draw::PAGE_H;

pub fn render_pdf(
    view: &ReportExportView,
    meta: &ExportMeta,
    branding: &ExportBranding,
) -> Result<RenderedExport, ReportExportError> {
    let (doc, page1, layer1) = PdfDocument::new("Report export", Mm(PAGE_W), Mm(PAGE_H), "Layer 1");
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|_| ReportExportError::RenderFailed)?;
    let bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|_| ReportExportError::RenderFailed)?;

    let mut layer = doc.get_page(page1).get_layer(layer1);
    let mut y = draw_header(&layer, &bold, branding, meta);
    y = draw_meta(&layer, &font, &bold, view, meta, y);
    y = draw_kpis(&layer, &font, &bold, view, y);
    y = draw_sales_heading(&layer, &bold, y, false);

    for (index, sale) in view.sales.iter().enumerate() {
        if index > 0 && index % PDF_ROWS_PER_PAGE == 0 {
            let (page, layer_ref) = doc.add_page(Mm(PAGE_W), Mm(PAGE_H), "Layer 1");
            layer = doc.get_page(page).get_layer(layer_ref);
            y = TOP_Y;
            y = draw_sales_heading(&layer, &bold, y, true);
        }
        let order = sale
            .order_id
            .map(pdf_draw::short_id)
            .unwrap_or_else(|| "-".to_owned());
        y = draw_table_row(
            &layer,
            &font,
            y,
            index,
            &[
                &pdf_draw::short_id(sale.sale_id),
                &order,
                &pdf_draw::short_id(sale.commerce_id),
                &crate::export::format::format_money_brl(sale.amount_cents, &sale.currency),
            ],
        );
    }

    if view.sales.is_empty() {
        y = draw_table_row(
            &layer,
            &font,
            y,
            0,
            &["No sales in this period", "", "", ""],
        );
    }

    y -= pdf_draw::LINE;
    draw_settlement(&layer, &font, &bold, view, y);
    draw_footer(&layer, &font, meta);

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
