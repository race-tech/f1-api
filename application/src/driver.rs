use diesel::{
    helper_types::{AsSelect, Eq, InnerJoin, InnerJoinOn, InnerJoinQuerySource, IntoBoxed, Select},
    prelude::*,
    sql_types::{Bool, Nullable},
    Identifiable, Queryable, Selectable,
};

use shared::models::DriverFilter;

use crate::{
    filters::*,
    pagination::{Paginate, Paginated},
    schema::{circuits, constructors, drivers, races, results},
};

#[derive(Identifiable, Queryable, Selectable, Debug)]
#[diesel(primary_key(driver_id))]
#[diesel(table_name = drivers, check_for_backend(super::Backend))]
pub struct Driver {
    pub driver_id: i32,
    pub driver_ref: String,
    pub number: Option<i32>,
    pub code: Option<String>,
    pub forename: String,
    pub surname: String,
    pub dob: Option<chrono::NaiveDate>,
    pub nationality: Option<String>,
    pub url: String,
}

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
        let limit = filter.limit.unwrap_or_default().inner() as i64;
        let page = filter.page.unwrap_or_default().inner() as i64;

        let conditions = fields_to_filter!(
            filter,
            driver_ref => (DriverRef, StringFilter::Equal),
            driver_number => (DriverNumber, NumberFilter::Equal),
            constructor => (Constructor, StringFilter::Equal),
            circuit => (Circuit, StringFilter::Equal),
            grid => (Grid, NumberFilter::Equal),
            result => (Result, NumberFilter::Equal)
        );

        let filter = match create_filter!(conditions, AndOr::And) {
            Some(boxed_condition) => boxed_condition,
            None => return Self::boxed().paginate(page).per_page(limit),
        };

        Self::boxed().filter(filter).paginate(page).per_page(limit)
    }
}

impl From<Driver> for shared::models::Driver {
    fn from(driver: Driver) -> shared::models::Driver {
        shared::models::Driver {
            driver_ref: driver.driver_ref,
            number: driver.number,
            code: driver.code,
            forename: driver.forename,
            surname: driver.surname,
            dob: driver.dob,
            nationality: driver.nationality,
            url: driver.url,
        }
    }
}

enum Condition {
    DriverRef(StringFilter),
    DriverNumber(NumberFilter<i32>),
    Constructor(StringFilter),
    Circuit(StringFilter),
    Grid(NumberFilter<i32>),
    Result(NumberFilter<i32>),
}

impl Condition {
    fn into_boxed_condition(self) -> Option<BoxedCondition> {
        Some(match self {
            Condition::Grid(f) => number_filter!(f, results::grid),
            Condition::Result(f) => number_filter!(f, results::position),
            Condition::Constructor(f) => string_filter!(f, constructors::constructor_ref),
            Condition::Circuit(f) => string_filter!(f, circuits::circuit_ref),
            Condition::DriverRef(f) => string_filter!(f, drivers::driver_ref),
            Condition::DriverNumber(f) => number_filter!(f, drivers::number),
        })
    }
}
