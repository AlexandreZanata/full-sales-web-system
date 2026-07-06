use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CursorPaginationMeta {
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Uuid>,
    pub has_more: bool,
    pub limit: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CursorListResponse<T> {
    pub data: Vec<T>,
    pub pagination: CursorPaginationMeta,
}

/// Builds a cursor page from `limit + 1` rows (standard over-fetch pattern).
pub fn build_cursor_page<T, F>(mut rows: Vec<T>, limit: u32, id_of: F) -> CursorListResponse<T>
where
    F: Fn(&T) -> Uuid,
{
    let limit_usize = limit as usize;
    let has_more = rows.len() > limit_usize;
    if has_more {
        rows.truncate(limit_usize);
    }
    let next_cursor = if has_more {
        rows.last().map(&id_of)
    } else {
        None
    };
    CursorListResponse {
        data: rows,
        pagination: CursorPaginationMeta {
            next_cursor,
            has_more,
            limit,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct Row {
        id: Uuid,
    }

    #[test]
    fn given_extra_row_when_build_cursor_page_then_has_more_and_next_cursor() {
        let ids: Vec<Uuid> = (0..3).map(|_| Uuid::now_v7()).collect();
        let rows: Vec<Row> = ids.iter().copied().map(|id| Row { id }).collect();
        let page = build_cursor_page(rows, 2, |r| r.id);
        assert_eq!(page.data.len(), 2);
        assert!(page.pagination.has_more);
        assert_eq!(page.pagination.next_cursor, Some(ids[1]));
    }

    #[test]
    fn given_short_page_when_build_cursor_page_then_no_more() {
        let id = Uuid::now_v7();
        let page = build_cursor_page(vec![Row { id }], 20, |r| r.id);
        assert!(!page.pagination.has_more);
        assert!(page.pagination.next_cursor.is_none());
    }
}
