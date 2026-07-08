//! Shared drawing helpers for official-style PDF export.

use printpdf::{Color, Mm, PdfLayerReference, Point, Rect, Rgb};

pub const PAGE_W: f32 = 210.0;
pub const PAGE_H: f32 = 297.0;
pub const MARGIN_X: f32 = 18.0;
pub const CONTENT_W: f32 = PAGE_W - MARGIN_X * 2.0;
pub const TOP_Y: f32 = 268.0;
pub const LINE: f32 = 5.0;

pub type Font = printpdf::IndirectFontRef;

pub fn brand() -> Color {
    Color::Rgb(Rgb::new(0.09, 0.20, 0.36, None))
}

pub fn ink() -> Color {
    Color::Rgb(Rgb::new(0.07, 0.07, 0.07, None))
}

pub fn muted() -> Color {
    Color::Rgb(Rgb::new(0.35, 0.35, 0.35, None))
}

pub fn header_bg() -> Color {
    Color::Rgb(Rgb::new(0.91, 0.93, 0.96, None))
}

pub fn stripe() -> Color {
    Color::Rgb(Rgb::new(0.96, 0.97, 0.98, None))
}

pub fn white() -> Color {
    Color::Rgb(Rgb::new(1.0, 1.0, 1.0, None))
}

pub fn fill_rect(layer: &PdfLayerReference, llx: f32, lly: f32, urx: f32, ury: f32, color: Color) {
    layer.set_fill_color(color);
    layer.add_rect(Rect::new(Mm(llx), Mm(lly), Mm(urx), Mm(ury)));
}

pub fn hline(layer: &PdfLayerReference, x1: f32, x2: f32, y: f32, color: Color) {
    layer.set_outline_color(color);
    layer.set_outline_thickness(0.4);
    layer.add_line(printpdf::Line {
        points: vec![
            (Point::new(Mm(x1), Mm(y)), false),
            (Point::new(Mm(x2), Mm(y)), false),
        ],
        is_closed: false,
    });
}

pub fn text(
    layer: &PdfLayerReference,
    font: &Font,
    size: f32,
    x: f32,
    y: f32,
    color: Color,
    value: &str,
) {
    layer.set_fill_color(color);
    layer.use_text(value, size, Mm(x), Mm(y), font);
}

pub fn short_id(id: uuid::Uuid) -> String {
    let raw = id.to_string();
    format!("{}…", &raw[..8.min(raw.len())])
}

pub fn write_cols(layer: &PdfLayerReference, font: &Font, y: f32, color: Color, cols: &[&str]) {
    let xs = [
        MARGIN_X + 2.0,
        MARGIN_X + 48.0,
        MARGIN_X + 92.0,
        MARGIN_X + 138.0,
    ];
    for (i, col) in cols.iter().enumerate() {
        if let Some(x) = xs.get(i) {
            text(layer, font, 8.0, *x, y, color.clone(), col);
        }
    }
}
