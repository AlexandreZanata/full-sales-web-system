use application::list_query::{FilterFieldSpec, ListFilterOp};

use super::parse::RouteListConfig;

const EQ: &[ListFilterOp] = &[ListFilterOp::Eq];
const DATE_OPS: &[ListFilterOp] = &[ListFilterOp::Gte, ListFilterOp::Lte];

static AUDIT_FILTERS: [FilterFieldSpec; 3] = [
    FilterFieldSpec::new("actor_id", EQ),
    FilterFieldSpec::new("action", EQ),
    FilterFieldSpec::new("created_at", DATE_OPS),
];

pub static AUDIT_EVENTS_LIST_CONFIG: RouteListConfig<'static> = RouteListConfig {
    filter_whitelist: &AUDIT_FILTERS,
    sort_whitelist: &[],
};
