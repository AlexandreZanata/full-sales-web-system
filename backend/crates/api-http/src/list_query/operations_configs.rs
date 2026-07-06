use application::list_query::{FilterFieldSpec, ListFilterOp};

use super::parse::RouteListConfig;

const EQ: &[ListFilterOp] = &[ListFilterOp::Eq];
const DATE_OPS: &[ListFilterOp] = &[ListFilterOp::Gte, ListFilterOp::Lte];

static SALES_FILTERS: [FilterFieldSpec; 4] = [
    FilterFieldSpec::new("commerce_id", EQ),
    FilterFieldSpec::new("driver_id", EQ),
    FilterFieldSpec::new("status", EQ),
    FilterFieldSpec::new("created_at", DATE_OPS),
];
static ORDER_FILTERS: [FilterFieldSpec; 3] = [
    FilterFieldSpec::new("status", EQ),
    FilterFieldSpec::new("commerce_id", EQ),
    FilterFieldSpec::new("created_at", DATE_OPS),
];
static PORTAL_ORDER_FILTERS: [FilterFieldSpec; 1] = [FilterFieldSpec::new("status", EQ)];
static DELIVERY_FILTERS: [FilterFieldSpec; 2] = [
    FilterFieldSpec::new("status", EQ),
    FilterFieldSpec::new("created_at", DATE_OPS),
];

pub static SALES_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &SALES_FILTERS,
    sort_whitelist: &[],
};

pub static ORDERS_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &ORDER_FILTERS,
    sort_whitelist: &[],
};

pub static PORTAL_ORDERS_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &PORTAL_ORDER_FILTERS,
    sort_whitelist: &[],
};

pub static DELIVERIES_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &DELIVERY_FILTERS,
    sort_whitelist: &[],
};
