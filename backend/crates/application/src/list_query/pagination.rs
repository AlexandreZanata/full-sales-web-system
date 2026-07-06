use uuid::Uuid;

use super::error::ListQueryError;

pub const DEFAULT_LIST_LIMIT: u32 = 20;
pub const MAX_LIST_LIMIT: u32 = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListPagination {
    pub limit: u32,
    pub cursor: Option<Uuid>,
}

impl ListPagination {
    pub fn new(limit: Option<u32>, cursor: Option<Uuid>) -> Result<Self, ListQueryError> {
        let limit = limit.unwrap_or(DEFAULT_LIST_LIMIT);
        if !(1..=MAX_LIST_LIMIT).contains(&limit) {
            return Err(ListQueryError::invalid_pagination(
                "limit",
                "limit must be between 1 and 100",
            ));
        }
        Ok(Self { limit, cursor })
    }

    pub fn fetch_size(&self) -> u32 {
        self.limit.saturating_add(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_no_limit_when_new_then_default_20() {
        let p = ListPagination::new(None, None).expect("valid");
        assert_eq!(p.limit, DEFAULT_LIST_LIMIT);
        assert!(p.cursor.is_none());
    }

    #[test]
    fn given_limit_over_max_when_new_then_invalid_pagination() {
        let err = ListPagination::new(Some(101), None).unwrap_err();
        assert!(matches!(
            err,
            ListQueryError::InvalidPagination { field: "limit", .. }
        ));
    }

    #[test]
    fn given_limit_zero_when_new_then_invalid_pagination() {
        let err = ListPagination::new(Some(0), None).unwrap_err();
        assert!(matches!(
            err,
            ListQueryError::InvalidPagination { field: "limit", .. }
        ));
    }

    #[test]
    fn given_cursor_when_new_then_stored() {
        let id = Uuid::now_v7();
        let p = ListPagination::new(Some(10), Some(id)).expect("valid");
        assert_eq!(p.cursor, Some(id));
        assert_eq!(p.fetch_size(), 11);
    }
}
