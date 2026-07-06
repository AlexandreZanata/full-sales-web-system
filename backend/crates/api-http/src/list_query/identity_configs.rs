use application::list_query::{FilterFieldSpec, ListFilterOp};

use super::parse::RouteListConfig;

const EQ: &[ListFilterOp] = &[ListFilterOp::Eq];

static COMMERCE_FILTERS: [FilterFieldSpec; 1] = [FilterFieldSpec::new("active", EQ)];
static USER_FILTERS: [FilterFieldSpec; 2] = [
    FilterFieldSpec::new("active", EQ),
    FilterFieldSpec::new("role", EQ),
];

pub static COMMERCES_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &COMMERCE_FILTERS,
    sort_whitelist: &[],
};

pub static USERS_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &USER_FILTERS,
    sort_whitelist: &[],
};

pub static COMMERCE_ADDRESSES_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &[],
    sort_whitelist: &[],
};
