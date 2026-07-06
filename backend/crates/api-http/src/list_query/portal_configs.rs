use application::list_query::{FilterFieldSpec, ListFilterOp};

use super::parse::RouteListConfig;

const EQ: &[ListFilterOp] = &[ListFilterOp::Eq];

static PORTAL_PRODUCT_FILTERS: [FilterFieldSpec; 1] =
    [FilterFieldSpec::new("category_slug", EQ)];

pub static PORTAL_PRODUCTS_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &PORTAL_PRODUCT_FILTERS,
    sort_whitelist: &[],
};

pub static PORTAL_CATEGORIES_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &[],
    sort_whitelist: &[],
};
