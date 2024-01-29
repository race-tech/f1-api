use diesel::{
    helper_types::{AsSelect, InnerJoin, InnerJoinQuerySource, IntoBoxed, Select},
    prelude::*,
    sql_types::{Bool, Nullable},
};
use shared::filters::DriverStandingFilter;

use crate::models::{Driver, DriverStanding};
use crate::prelude::*;

type BoxedConditionSource = InnerJoinQuerySource<drivers::table, driverStandings::table>;
type BoxedCondition =
    Box<dyn BoxableExpression<BoxedConditionSource, super::Backend, SqlType = Nullable<Bool>>>;

type BoxedQuerySource = InnerJoin<drivers::table, driverStandings::table>;
type BoxedQuery = IntoBoxed<
    'static,
    Select<BoxedQuerySource, AsSelect<(DriverStanding, Driver), super::Backend>>,
    super::Backend,
>;

impl DriverStanding {
    fn boxed() -> BoxedQuery {
        drivers::table
            .inner_join(driverStandings::table)
            .select(<(DriverStanding, Driver)>::as_select())
            .distinct()
            .order(driverStandings::driver_standing_id.asc())
            .into_boxed()
    }

    pub fn filter(filter: DriverStandingFilter) -> Paginated<BoxedQuery> {
        let limit = filter.limit.unwrap_or_default().0 as i64;
        let page = filter.page.unwrap_or_default().0 as i64;

        let conditions = fields_to_filter!(
            filter,
            name => (DriverRef, StringFilter::Equal),
            result => (Result, NumberFilter::Equal),
            race_id => (RaceId, NumberFilter::Equal)
        );

        let filter = match create_filter!(conditions, AndOr::And) {
            Some(boxed_condition) => boxed_condition,
            None => return Self::boxed().paginate(page).per_page(limit),
        };

        Self::boxed().filter(filter).paginate(page).per_page(limit)
    }
}

enum Condition {
    Result(NumberFilter<i32>),
    DriverRef(StringFilter),
    RaceId(NumberFilter<i32>),
}

impl Condition {
    fn into_boxed_condition(self) -> Option<BoxedCondition> {
        use Condition::*;

        Some(match self {
            DriverRef(f) => string_filter!(f, drivers::driver_ref),
            Result(f) => number_filter!(f, driverStandings::position),
            RaceId(f) => number_filter!(f, driverStandings::race_id),
        })
    }
}
