use application::list_query::{FilterFieldSpec, ListFilterOp};

use super::parse::RouteListConfig;

const EQ: &[ListFilterOp] = &[ListFilterOp::Eq];
const LIKE: &[ListFilterOp] = &[ListFilterOp::Like];
const DATE_OPS: &[ListFilterOp] = &[ListFilterOp::Gte, ListFilterOp::Lte];

static PRODUCT_FILTERS: [FilterFieldSpec; 1] = [FilterFieldSpec::new("active", EQ)];
static CATEGORY_FILTERS: [FilterFieldSpec; 1] = [FilterFieldSpec::new("active", EQ)];
static BALANCE_FILTERS: [FilterFieldSpec; 2] = [
    FilterFieldSpec::new("name", LIKE),
    FilterFieldSpec::new("sku", LIKE),
];
static MOVEMENT_FILTERS: [FilterFieldSpec; 1] = [FilterFieldSpec::new("created_at", DATE_OPS)];

pub static PRODUCTS_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &PRODUCT_FILTERS,
    sort_whitelist: &[],
};

pub static CATEGORIES_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &CATEGORY_FILTERS,
    sort_whitelist: &[],
};

pub static PRODUCT_IMAGES_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &[],
    sort_whitelist: &[],
};

pub static STOCK_BALANCES_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &BALANCE_FILTERS,
    sort_whitelist: &[],
};

pub static STOCK_MOVEMENTS_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &MOVEMENT_FILTERS,
    sort_whitelist: &[],
};
