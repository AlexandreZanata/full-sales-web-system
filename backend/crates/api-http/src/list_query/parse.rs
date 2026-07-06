use application::list_query::{
    FilterFieldSpec, ListFilter, ListFilterOp, ListPagination, ListQueryError, ParsedListQuery,
    parse_sort_param, validate_filters, validate_sorts,
};
use uuid::Uuid;

use super::error::ListQueryApiError;

#[derive(Debug, Clone)]
pub struct RouteListConfig<'a> {
    pub filter_whitelist: &'a [FilterFieldSpec],
    pub sort_whitelist: &'a [&'static str],
}

pub fn parse_list_query(
    pairs: &[(String, String)],
    config: &RouteListConfig<'_>,
) -> Result<ParsedListQuery, ListQueryApiError> {
    let mut limit: Option<u32> = None;
    let mut cursor: Option<Uuid> = None;
    let mut filters = Vec::new();
    let mut sort_raw: Option<String> = None;

    for (key, value) in pairs {
        match key.as_str() {
            "limit" => {
                limit = Some(parse_limit(value)?);
            }
            "cursor" => {
                cursor = Some(parse_cursor(value)?);
            }
            "sort" => {
                sort_raw = Some(value.clone());
            }
            _ if key.starts_with("filter[") => {
                filters.push(parse_filter_key_value(key, value)?);
            }
            _ => {}
        }
    }

    let pagination =
        ListPagination::new(limit, cursor).map_err(ListQueryApiError::from_list_query_error)?;
    validate_filters(&filters, config.filter_whitelist)
        .map_err(ListQueryApiError::from_list_query_error)?;
    let sorts = sort_raw
        .as_deref()
        .map(parse_sort_param)
        .unwrap_or_default();
    validate_sorts(&sorts, config.sort_whitelist)
        .map_err(ListQueryApiError::from_list_query_error)?;

    Ok(ParsedListQuery {
        pagination,
        filters,
        sorts,
    })
}

fn parse_limit(raw: &str) -> Result<u32, ListQueryApiError> {
    raw.parse::<u32>().map_err(|_| {
        ListQueryApiError::from_list_query_error(ListQueryError::invalid_pagination(
            "limit",
            "limit must be a positive integer",
        ))
    })
}

fn parse_cursor(raw: &str) -> Result<Uuid, ListQueryApiError> {
    Uuid::parse_str(raw).map_err(|_| {
        ListQueryApiError::from_list_query_error(ListQueryError::invalid_pagination(
            "cursor",
            "cursor must be a valid UUID",
        ))
    })
}

fn parse_filter_key_value(key: &str, value: &str) -> Result<ListFilter, ListQueryApiError> {
    let inner = key
        .strip_prefix("filter[")
        .and_then(|s| s.strip_suffix(']'))
        .ok_or_else(|| invalid_filter_syntax(key))?;

    let (field, op_name) = match inner.split_once("][") {
        Some((field, op)) => (field, op.strip_suffix(']').unwrap_or(op)),
        None => (inner, "eq"),
    };

    if field.is_empty() {
        return Err(invalid_filter_syntax(key));
    }

    let op = ListFilterOp::parse(op_name).map_err(|mut err| {
        if let ListQueryError::InvalidFilterOperator { op, .. } = &mut err {
            let captured = op.clone();
            ListQueryApiError::from_list_query_error(ListQueryError::InvalidFilterOperator {
                field: field.to_owned(),
                op: captured,
            })
        } else {
            ListQueryApiError::from_list_query_error(err)
        }
    })?;

    Ok(ListFilter {
        field: field.to_owned(),
        op,
        value: value.to_owned(),
    })
}

fn invalid_filter_syntax(key: &str) -> ListQueryApiError {
    ListQueryApiError::from_list_query_error(ListQueryError::InvalidFilterField {
        field: key.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTIVE_OPS: &[ListFilterOp] = &[ListFilterOp::Eq];
    static FILTER_WHITELIST: [FilterFieldSpec; 1] =
        [FilterFieldSpec::new("active", ACTIVE_OPS)];
    static SORT_WHITELIST: [&str; 1] = ["name"];
    static SAMPLE_CONFIG: RouteListConfig<'static> = RouteListConfig {
        filter_whitelist: &FILTER_WHITELIST,
        sort_whitelist: &SORT_WHITELIST,
    };

    #[test]
    fn given_limit_and_cursor_when_parse_then_pagination_vo() {
        let id = Uuid::now_v7();
        let pairs = vec![
            ("limit".into(), "10".into()),
            ("cursor".into(), id.to_string()),
        ];
        let parsed = parse_list_query(&pairs, &SAMPLE_CONFIG).expect("parse");
        assert_eq!(parsed.pagination.limit, 10);
        assert_eq!(parsed.pagination.cursor, Some(id));
    }

    #[test]
    fn given_filter_active_eq_when_parse_then_filter_vo() {
        let pairs = vec![("filter[active]".into(), "true".into())];
        let parsed = parse_list_query(&pairs, &SAMPLE_CONFIG).expect("parse");
        assert_eq!(parsed.filters.len(), 1);
        assert_eq!(parsed.filters[0].field, "active");
        assert_eq!(parsed.filters[0].op, ListFilterOp::Eq);
    }

    #[test]
    fn given_unknown_filter_when_parse_then_invalid_filter_field() {
        let pairs = vec![("filter[secret]".into(), "x".into())];
        let err = parse_list_query(&pairs, &SAMPLE_CONFIG).unwrap_err();
        assert_eq!(err.code, "invalid_filter_field");
    }

    #[test]
    fn given_invalid_sort_when_parse_then_invalid_sort_field() {
        let pairs = vec![("sort".into(), "-created_at".into())];
        let err = parse_list_query(&pairs, &SAMPLE_CONFIG).unwrap_err();
        assert_eq!(err.code, "invalid_sort_field");
    }

    #[test]
    fn given_limit_200_when_parse_then_invalid_pagination() {
        let pairs = vec![("limit".into(), "200".into())];
        let err = parse_list_query(&pairs, &SAMPLE_CONFIG).unwrap_err();
        assert_eq!(err.code, "invalid_pagination");
        assert_eq!(err.field, "limit");
    }
}
