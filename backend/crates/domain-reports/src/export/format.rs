use chrono::{DateTime, Utc};

pub fn format_date_pt_br(value: DateTime<Utc>) -> String {
    value.format("%d/%m/%Y").to_string()
}

pub fn format_datetime_pt_br(value: DateTime<Utc>) -> String {
    value.format("%d/%m/%Y %H:%M").to_string()
}

pub fn format_money_brl(cents: i64, currency: &str) -> String {
    if currency != "BRL" {
        return format!("{:.2} {currency}", cents as f64 / 100.0);
    }

    let negative = cents < 0;
    let abs = cents.unsigned_abs();
    let reais = abs / 100;
    let centavos = abs % 100;
    let body = format!("R$ {reais},{centavos:02}");
    if negative { format!("-{body}") } else { body }
}

pub fn period_start_filename(value: DateTime<Utc>) -> String {
    value.format("%Y-%m-%d").to_string()
}

pub fn escape_csv_field(value: &str) -> String {
    if value.contains(['"', ',', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}
