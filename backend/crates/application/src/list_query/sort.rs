use super::error::ListQueryError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListSortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSort {
    pub field: String,
    pub direction: ListSortDirection,
}

pub fn validate_sorts(sorts: &[ListSort], whitelist: &[&str]) -> Result<(), ListQueryError> {
    for sort in sorts {
        if !whitelist.contains(&sort.field.as_str()) {
            return Err(ListQueryError::InvalidSortField {
                field: sort.field.clone(),
            });
        }
    }
    Ok(())
}

pub fn parse_sort_param(raw: &str) -> Vec<ListSort> {
    if raw.trim().is_empty() {
        return Vec::new();
    }
    raw.split(',')
        .filter_map(|part| {
            let part = part.trim();
            if part.is_empty() {
                return None;
            }
            let (field, direction) = if let Some(rest) = part.strip_prefix('-') {
                (rest, ListSortDirection::Desc)
            } else {
                (part, ListSortDirection::Asc)
            };
            Some(ListSort {
                field: field.to_owned(),
                direction,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_desc_prefix_when_parse_sort_then_descending() {
        let sorts = parse_sort_param("-created_at,name");
        assert_eq!(sorts.len(), 2);
        assert_eq!(sorts[0].field, "created_at");
        assert_eq!(sorts[0].direction, ListSortDirection::Desc);
        assert_eq!(sorts[1].direction, ListSortDirection::Asc);
    }

    #[test]
    fn given_unknown_sort_field_when_validate_then_invalid_sort_field() {
        let sorts = vec![ListSort {
            field: "secret".into(),
            direction: ListSortDirection::Asc,
        }];
        let err = validate_sorts(&sorts, &["name"]).unwrap_err();
        assert!(matches!(err, ListQueryError::InvalidSortField { .. }));
    }
}
