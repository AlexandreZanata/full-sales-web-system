use application::list_query::{ListFilter, ListFilterOp};
use uuid::Uuid;

pub fn filter_eq_string(filters: &[ListFilter], field: &str) -> Option<String> {
    filters
        .iter()
        .find(|f| f.field == field && f.op == ListFilterOp::Eq)
        .map(|f| f.value.clone())
}

pub fn filter_eq_uuid(filters: &[ListFilter], field: &str) -> Option<Uuid> {
    filter_eq_string(filters, field).and_then(|value| Uuid::parse_str(value.trim()).ok())
}

pub fn filter_eq_bool(filters: &[ListFilter], field: &str) -> Option<bool> {
    filters
        .iter()
        .find(|f| f.field == field && f.op == ListFilterOp::Eq)
        .and_then(|f| match f.value.as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        })
}

pub fn filter_like_pattern(filters: &[ListFilter], field: &str) -> Option<String> {
    filters
        .iter()
        .find(|f| f.field == field && f.op == ListFilterOp::Like)
        .map(|f| format!("%{}%", f.value.trim()))
        .filter(|p| p.len() > 2)
}

pub fn filter_gte_datetime(
    filters: &[ListFilter],
    field: &str,
) -> Option<chrono::DateTime<chrono::Utc>> {
    filter_datetime(filters, field, ListFilterOp::Gte)
}

pub fn filter_lte_datetime(
    filters: &[ListFilter],
    field: &str,
) -> Option<chrono::DateTime<chrono::Utc>> {
    filter_datetime(filters, field, ListFilterOp::Lte)
}

fn filter_datetime(
    filters: &[ListFilter],
    field: &str,
    op: ListFilterOp,
) -> Option<chrono::DateTime<chrono::Utc>> {
    filters
        .iter()
        .find(|f| f.field == field && f.op == op)
        .and_then(|f| chrono::DateTime::parse_from_rfc3339(f.value.trim()).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_active_filter_when_eq_bool_then_some() {
        let filters = vec![ListFilter {
            field: "active".into(),
            op: ListFilterOp::Eq,
            value: "false".into(),
        }];
        assert_eq!(filter_eq_bool(&filters, "active"), Some(false));
    }
}
