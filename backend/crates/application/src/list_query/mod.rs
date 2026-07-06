mod error;
mod filter;
mod pagination;
mod sort;

pub use error::ListQueryError;
pub use filter::{FilterFieldSpec, ListFilter, ListFilterOp, validate_filters};
pub use pagination::{ListPagination, DEFAULT_LIST_LIMIT, MAX_LIST_LIMIT};
pub use sort::{ListSort, ListSortDirection, parse_sort_param, validate_sorts};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedListQuery {
    pub pagination: ListPagination,
    pub filters: Vec<ListFilter>,
    pub sorts: Vec<ListSort>,
}
