use diesel::{
    alias,
    helper_types::{
        And, AsSelect, Eq, InnerJoin, InnerJoinOn, InnerJoinQuerySource, IntoBoxed, LeftJoinOn,
        LeftJoinQuerySource, Lt, Select,
    },
    prelude::*,
    query_source::{Alias, AliasedField},
    sql_types::{Bool, Nullable},
};
use shared::filters::DriverStandingFilter;

use crate::models::DriverStanding;
use crate::prelude::*;

type BoxedConditionSource = InnerJoinQuerySource<
    InnerJoinQuerySource<
        InnerJoinQuerySource<
            InnerJoinQuerySource<
                LeftJoinQuerySource<
                    races::table,
                    Alias<RaceAlias>,
                    And<
                        Eq<races::year, AliasedField<RaceAlias, races::year>>,
                        Lt<races::round, AliasedField<RaceAlias, races::round>>,
                    >,
                >,
                driverStandings::table,
            >,
            drivers::table,
            Eq<drivers::driver_id, driverStandings::driver_id>,
        >,
        results::table,
        And<Eq<drivers::driver_id, results::driver_id>, Eq<races::race_id, results::race_id>>,
    >,
    constructors::table,
    Eq<results::constructor_id, constructors::constructor_id>,
>;
type BoxedCondition =
    Box<dyn BoxableExpression<BoxedConditionSource, super::Backend, SqlType = Nullable<Bool>>>;

type BoxedQuerySource = InnerJoinOn<
    InnerJoinOn<
        InnerJoinOn<
            InnerJoin<
                LeftJoinOn<
                    races::table,
                    Alias<RaceAlias>,
                    And<
                        Eq<races::year, AliasedField<RaceAlias, races::year>>,
                        Lt<races::round, AliasedField<RaceAlias, races::round>>,
                    >,
                >,
                driverStandings::table,
            >,
            drivers::table,
            Eq<drivers::driver_id, driverStandings::driver_id>,
        >,
        results::table,
        And<Eq<drivers::driver_id, results::driver_id>, Eq<races::race_id, results::race_id>>,
    >,
    constructors::table,
    Eq<results::constructor_id, constructors::constructor_id>,
>;
pub type BoxedQuery = IntoBoxed<
    'static,
    Select<BoxedQuerySource, AsSelect<DriverStanding, super::Backend>>,
    super::Backend,
>;

alias!(races as r1: RaceAlias);

impl DriverStanding {
    fn boxed() -> BoxedQuery {
        races::table
            .left_join(
                r1.on(races::year
                    .eq(r1.field(races::year))
                    .and(races::round.lt(r1.field(races::round)))),
            )
            .inner_join(driverStandings::table)
            .inner_join(drivers::table.on(drivers::driver_id.eq(driverStandings::driver_id)))
            .inner_join(
                results::table.on(drivers::driver_id
                    .eq(results::driver_id)
                    .and(races::race_id.eq(results::race_id))),
            )
            .inner_join(
                constructors::table.on(results::constructor_id.eq(constructors::constructor_id)),
            )
            .select(DriverStanding::as_select())
            .into_boxed()
    }

    pub fn filter(filter: DriverStandingFilter) -> Paginated<BoxedQuery> {
        let limit = filter.limit.unwrap_or_default().0 as i64;
        let page = filter.page.unwrap_or_default().0 as i64;

        let mut conditions = fields_to_filter!(
            filter,
            name => (DriverRef, StringFilter::Equal),
            result => (Result, NumberFilter::Equal)
        );
        conditions.push(Condition::RaceIdNull);

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
    RaceIdNull,
}

impl Condition {
    fn into_boxed_condition(self) -> Option<BoxedCondition> {
        use Condition::*;

        Some(match self {
            DriverRef(f) => string_filter!(f, drivers::driver_ref),
            Result(f) => number_filter!(f, driverStandings::position),
            RaceIdNull => Box::new(r1.fields(races::race_id).is_null().nullable()),
        })
    }
}
