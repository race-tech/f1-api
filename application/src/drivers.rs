use diesel::{
    helper_types::{
        AsSelect, Eq, Filter, InnerJoin, InnerJoinOn, InnerJoinQuerySource, IntoBoxed, Select,
    },
    prelude::*,
    sql_types::{Bool, Nullable},
};

use shared::{filters::DriverFilter, parameters::DriverId};

use crate::models::Driver;
use crate::prelude::*;

type All = Select<drivers::table, AsSelect<Driver, super::Backend>>;
type ByDriverId = Filter<All, diesel::dsl::Eq<drivers::driver_id, i32>>;

type BoxedConditionSource = InnerJoinQuerySource<
    InnerJoinQuerySource<
        InnerJoinQuerySource<
            InnerJoinQuerySource<results::table, drivers::table>,
            constructors::table,
        >,
        races::table,
    >,
    circuits::table,
    Eq<circuits::circuit_id, races::circuit_id>,
>;
type BoxedCondition =
    Box<dyn BoxableExpression<BoxedConditionSource, super::Backend, SqlType = Nullable<Bool>>>;

type BoxedQuerySource = InnerJoinOn<
    InnerJoin<
        InnerJoin<InnerJoin<results::table, drivers::table>, constructors::table>,
        races::table,
    >,
    circuits::table,
    Eq<circuits::circuit_id, races::circuit_id>,
>;
type BoxedQuery =
    IntoBoxed<'static, Select<BoxedQuerySource, AsSelect<Driver, super::Backend>>, super::Backend>;

impl Driver {
    fn boxed() -> BoxedQuery {
        results::table
            .inner_join(drivers::table)
            .inner_join(constructors::table)
            .inner_join(races::table)
            .inner_join(circuits::table.on(circuits::circuit_id.eq(races::circuit_id)))
            .select(Driver::as_select())
            .distinct()
            .order(drivers::driver_id.asc())
            .into_boxed()
    }

    pub fn filter(filter: DriverFilter) -> Paginated<BoxedQuery> {
        let limit = filter.limit.unwrap_or_default().0 as i64;
        let page = filter.page.unwrap_or_default().0 as i64;

        let conditions = fields_to_filter!(
            filter,
            driver_ref => (DriverRef, StringFilter::Equal),
            driver_number => (DriverNumber, NumberFilter::Equal),
            constructor => (Constructor, StringFilter::Equal),
            circuit => (Circuit, StringFilter::Equal),
            grid => (Grid, NumberFilter::Equal),
            result => (Result, NumberFilter::Equal),
            year => (Year, NumberFilter::Equal),
            round => (Round, NumberFilter::Equal)
        );

        let filter = match create_filter!(conditions, AndOr::And) {
            Some(boxed_condition) => boxed_condition,
            None => return Self::boxed().paginate(page).per_page(limit),
        };

        Self::boxed().filter(filter).paginate(page).per_page(limit)
    }

    pub fn by_id(driver_id: DriverId) -> ByDriverId {
        drivers::table
            .filter(drivers::driver_id.eq(driver_id.0))
            .select(Driver::as_select())
    }
}

enum Condition {
    DriverRef(StringFilter),
    DriverNumber(NumberFilter<i32>),
    Constructor(StringFilter),
    Circuit(StringFilter),
    Grid(NumberFilter<i32>),
    Result(NumberFilter<i32>),
    Year(NumberFilter<i32>),
    Round(NumberFilter<i32>),
}

impl Condition {
    fn into_boxed_condition(self) -> Option<BoxedCondition> {
        use Condition::*;

        Some(match self {
            Grid(f) => number_filter!(f, results::grid),
            Result(f) => number_filter!(f, results::position),
            Constructor(f) => string_filter!(f, constructors::constructor_ref),
            Circuit(f) => string_filter!(f, circuits::circuit_ref),
            DriverRef(f) => string_filter!(f, drivers::driver_ref),
            DriverNumber(f) => number_filter!(f, drivers::number),
            Year(f) => number_filter!(f, races::year),
            Round(f) => number_filter!(f, races::round),
        })
    }
}
