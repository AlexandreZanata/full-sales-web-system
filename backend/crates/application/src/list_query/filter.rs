use super::error::ListQueryError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListFilterOp {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    Like,
    IsNull,
}

impl ListFilterOp {
    pub fn parse(name: &str) -> Result<Self, ListQueryError> {
        match name {
            "eq" | "" => Ok(Self::Eq),
            "ne" => Ok(Self::Ne),
            "gt" => Ok(Self::Gt),
            "gte" => Ok(Self::Gte),
            "lt" => Ok(Self::Lt),
            "lte" => Ok(Self::Lte),
            "in" => Ok(Self::In),
            "like" => Ok(Self::Like),
            "is_null" => Ok(Self::IsNull),
            other => Err(ListQueryError::InvalidFilterOperator {
                field: String::new(),
                op: other.to_owned(),
            }),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eq => "eq",
            Self::Ne => "ne",
            Self::Gt => "gt",
            Self::Gte => "gte",
            Self::Lt => "lt",
            Self::Lte => "lte",
            Self::In => "in",
            Self::Like => "like",
            Self::IsNull => "is_null",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListFilter {
    pub field: String,
    pub op: ListFilterOp,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct FilterFieldSpec {
    pub field: &'static str,
    pub ops: &'static [ListFilterOp],
}

impl FilterFieldSpec {
    pub const fn new(field: &'static str, ops: &'static [ListFilterOp]) -> Self {
        Self { field, ops }
    }
}

pub fn validate_filters(
    filters: &[ListFilter],
    whitelist: &[FilterFieldSpec],
) -> Result<(), ListQueryError> {
    for filter in filters {
        let spec = whitelist
            .iter()
            .find(|w| w.field == filter.field)
            .ok_or_else(|| ListQueryError::InvalidFilterField {
                field: filter.field.clone(),
            })?;
        if !spec.ops.contains(&filter.op) {
            return Err(ListQueryError::InvalidFilterOperator {
                field: filter.field.clone(),
                op: filter.op.as_str().to_owned(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTIVE_OPS: &[ListFilterOp] = &[ListFilterOp::Eq];

    #[test]
    fn given_whitelisted_field_when_validate_then_ok() {
        let filters = vec![ListFilter {
            field: "active".into(),
            op: ListFilterOp::Eq,
            value: "true".into(),
        }];
        let whitelist = [FilterFieldSpec::new("active", ACTIVE_OPS)];
        assert!(validate_filters(&filters, &whitelist).is_ok());
    }

    #[test]
    fn given_unknown_field_when_validate_then_invalid_filter_field() {
        let filters = vec![ListFilter {
            field: "unknown".into(),
            op: ListFilterOp::Eq,
            value: "x".into(),
        }];
        let err = validate_filters(&filters, &[]).unwrap_err();
        assert!(matches!(err, ListQueryError::InvalidFilterField { .. }));
    }

    #[test]
    fn given_disallowed_op_when_validate_then_invalid_filter_operator() {
        let filters = vec![ListFilter {
            field: "active".into(),
            op: ListFilterOp::Like,
            value: "x".into(),
        }];
        let whitelist = [FilterFieldSpec::new("active", ACTIVE_OPS)];
        let err = validate_filters(&filters, &whitelist).unwrap_err();
        assert!(matches!(err, ListQueryError::InvalidFilterOperator { .. }));
    }
}
