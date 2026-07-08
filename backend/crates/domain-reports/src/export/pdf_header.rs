//! Brand header + meta/KPI sections for report PDF.

use printpdf::PdfLayerReference;

use crate::export::format::{format_date_pt_br, format_money_brl};
use crate::export::pdf_draw::{
    self, CONTENT_W, Font, LINE, MARGIN_X, PAGE_H, PAGE_W, TOP_Y,
};
use crate::export::view::{ExportBranding, ExportMeta, ReportExportView};

pub fn draw_header(
    layer: &PdfLayerReference,
    bold: &Font,
    branding: &ExportBranding,
    meta: &ExportMeta,
) -> f32 {
    pdf_draw::fill_rect(layer, 0.0, 272.0, PAGE_W, PAGE_H, pdf_draw::brand());
    pdf_draw::text(
        layer,
        bold,
        14.0,
        MARGIN_X,
        286.0,
        pdf_draw::white(),
        &branding.display_name,
    );
    pdf_draw::text(
        layer,
        bold,
        10.0,
        MARGIN_X,
        278.0,
        pdf_draw::white(),
        &format!("{} report", meta.report_type),
    );
    TOP_Y
}

pub fn draw_meta(
    layer: &PdfLayerReference,
    font: &Font,
    bold: &Font,
    view: &ReportExportView,
    meta: &ExportMeta,
    mut y: f32,
) -> f32 {
    let rows = [
        ("Report ID", meta.report_id.to_string()),
        ("Report type", meta.report_type.clone()),
        (
            "Period",
            format!(
                "{} – {}",
                format_date_pt_br(view.period.start),
                format_date_pt_br(view.period.end)
            ),
        ),
        ("Driver ID", view.driver_id.to_string()),
    ];
    for (label, value) in rows {
        pdf_draw::text(layer, bold, 9.0, MARGIN_X, y, pdf_draw::ink(), label);
        pdf_draw::text(layer, font, 9.0, MARGIN_X + 32.0, y, pdf_draw::ink(), &value);
        y -= LINE;
    }
    y - LINE * 0.5
}

pub fn draw_kpis(
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
        "Executive summary",
    );
    y -= LINE + 1.0;
    let gap = 4.0;
    let card_w = (CONTENT_W - gap) / 2.0;
    let card_h = 16.0;
    let bottom = y - card_h;
    let cards = [
        (MARGIN_X, view.sales.len().to_string(), "Sales in period"),
        (
            MARGIN_X + card_w + gap,
            format_money_brl(
                view.settlement.total_declared_cents,
                &view.settlement.currency,
            ),
            "Total declared",
        ),
    ];
    for (x, value, label) in cards {
        pdf_draw::fill_rect(layer, x, bottom, x + card_w, y, pdf_draw::header_bg());
        pdf_draw::text(layer, bold, 12.0, x + 3.0, y - 6.0, pdf_draw::brand(), &value);
        pdf_draw::text(layer, font, 8.0, x + 3.0, y - 12.0, pdf_draw::muted(), label);
    }
    bottom - LINE
}
