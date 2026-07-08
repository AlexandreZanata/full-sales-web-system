//! Sales table, settlement block, and footer for report PDF.

use printpdf::PdfLayerReference;

use crate::export::format::format_money_brl;
use crate::export::pdf_draw::{self, CONTENT_W, Font, LINE, MARGIN_X};
use crate::export::view::{ExportMeta, ReportExportView};

pub fn draw_sales_heading(
    layer: &PdfLayerReference,
    bold: &Font,
    mut y: f32,
    continued: bool,
) -> f32 {
    let title = if continued {
        "Sales (continued)"
    } else {
        "Sales"
    };
    pdf_draw::text(layer, bold, 11.0, MARGIN_X, y, pdf_draw::ink(), title);
    y -= LINE + 1.0;
    let row_h = 6.0;
    pdf_draw::fill_rect(
        layer,
        MARGIN_X,
        y - row_h,
        MARGIN_X + CONTENT_W,
        y,
        pdf_draw::header_bg(),
    );
    pdf_draw::write_cols(
        layer,
        bold,
        y - 4.5,
        pdf_draw::ink(),
        &["Sale ID", "Order ID", "Commerce", "Amount"],
    );
    y - row_h
}

pub fn draw_table_row(
    layer: &PdfLayerReference,
    font: &Font,
    y: f32,
    index: usize,
    cols: &[&str],
) -> f32 {
    let row_h = 5.5;
    if index % 2 == 1 {
        pdf_draw::fill_rect(
            layer,
            MARGIN_X,
            y - row_h,
            MARGIN_X + CONTENT_W,
            y,
            pdf_draw::stripe(),
        );
    }
    pdf_draw::hline(
        layer,
        MARGIN_X,
        MARGIN_X + CONTENT_W,
        y - row_h,
        pdf_draw::muted(),
    );
    pdf_draw::write_cols(layer, font, y - 4.0, pdf_draw::ink(), cols);
    y - row_h
}

pub fn draw_settlement(
    layer: &PdfLayerReference,
    font: &Font,
    bold: &Font,
    view: &ReportExportView,
    mut y: f32,
) -> f32 {
    pdf_draw::text(
        layer,
        bold,
        11.0,
        MARGIN_X,
        y,
        pdf_draw::ink(),
        "Declared settlement",
    );
    y -= LINE;
    pdf_draw::text(
        layer,
        font,
        10.0,
        MARGIN_X,
        y,
        pdf_draw::ink(),
        &format!(
            "Total declared: {}",
            format_money_brl(
                view.settlement.total_declared_cents,
                &view.settlement.currency
            )
        ),
    );
    y -= LINE;
    for (method, amount) in &view.settlement.by_payment_method {
        pdf_draw::text(
            layer,
            font,
            9.0,
            MARGIN_X,
            y,
            pdf_draw::ink(),
            &format!(
                "{method}: {}",
                format_money_brl(*amount, &view.settlement.currency)
            ),
        );
        y -= LINE;
    }
    pdf_draw::text(
        layer,
        font,
        8.0,
        MARGIN_X,
        y,
        pdf_draw::muted(),
        &view.settlement.disclaimer,
    );
    y - LINE
}

pub fn draw_footer(layer: &PdfLayerReference, font: &Font, meta: &ExportMeta) {
    let y = 18.0;
    pdf_draw::hline(
        layer,
        MARGIN_X,
        MARGIN_X + CONTENT_W,
        y + 6.0,
        pdf_draw::muted(),
    );
    pdf_draw::text(
        layer,
        font,
        8.0,
        MARGIN_X,
        y,
        pdf_draw::muted(),
        &format!("Document: {} · Full Sales", meta.report_id),
    );
    if let Some(url) = &meta.verify_url {
        pdf_draw::text(
            layer,
            font,
            8.0,
            MARGIN_X,
            y - LINE,
            pdf_draw::muted(),
            &format!("Verify: {url}"),
        );
    }
}
