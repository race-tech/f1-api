use diesel::prelude::*;

use shared::filters::ConstructorFilter;
use shared::models::Pagination;
use types::*;

use crate::models::Constructor;
use crate::prelude::*;

pub struct ConstructorBuilder(ConstructorFilter);

impl ConstructorBuilder {
    pub fn new(filter: ConstructorFilter) -> Self {
        Self(filter)
    }

    pub fn load(
        self,
        conn: &mut MysqlConnection,
    ) -> Result<(Vec<Constructor>, Pagination), diesel::result::Error> {
        Self::filter(self.0).load_and_count_pages(conn)
    }

    fn boxed() -> BoxedQuery {
        results::table
            .inner_join(drivers::table)
            .inner_join(constructors::table)
            .inner_join(races::table)
            .inner_join(circuits::table.on(circuits::circuit_id.eq(races::circuit_id)))
            .select(Constructor::as_select())
            .distinct()
            .order(constructors::constructor_id.asc())
            .into_boxed()
    }

    fn filter(filter: ConstructorFilter) -> Paginated<BoxedQuery> {
        let limit = filter.limit.unwrap_or_default().0 as i64;
        let page = filter.page.unwrap_or_default().0 as i64;

        let conditions = fields_to_filter!(
            filter,
            driver_ref => (DriverRef, StringFilter::Equal),
            driver_number => (DriverNumber, NumberFilter::Equal),
            constructor_ref => (Constructor, StringFilter::Equal),
            circuit_ref => (Circuit, StringFilter::Equal),
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

mod types {
    use diesel::{
        helper_types::{
            AsSelect, Eq, InnerJoin, InnerJoinOn, InnerJoinQuerySource, IntoBoxed, Select,
        },
        prelude::*,
        sql_types::{Bool, Nullable},
    };

    use crate::models::Constructor;
    use crate::prelude::*;

    pub type BoxedConditionSource = InnerJoinQuerySource<
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
    pub type BoxedCondition =
        Box<dyn BoxableExpression<BoxedConditionSource, crate::Backend, SqlType = Nullable<Bool>>>;

    pub type BoxedQuerySource = InnerJoinOn<
        InnerJoin<
            InnerJoin<InnerJoin<results::table, drivers::table>, constructors::table>,
            races::table,
        >,
        circuits::table,
        Eq<circuits::circuit_id, races::circuit_id>,
    >;
    pub type BoxedQuery = IntoBoxed<
        'static,
        Select<BoxedQuerySource, AsSelect<Constructor, crate::Backend>>,
        crate::Backend,
    >;
}
