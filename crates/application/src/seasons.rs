use diesel::prelude::*;

use shared::filters::SeasonFilter;

use shared::models::Pagination;
use types::*;

use crate::models::Season;
use crate::prelude::*;

pub struct SeasonBuilder(SeasonFilter);

impl SeasonBuilder {
    pub fn new(filter: SeasonFilter) -> Self {
        Self(filter)
    }

    pub fn load(
        self,
        conn: &mut MysqlConnection,
    ) -> Result<(Vec<Season>, Pagination), diesel::result::Error> {
        Self::filter(self.0).load_and_count_pages(conn)
    }

    fn boxed() -> BoxedQuery {
        seasons::table
            .inner_join(races::table)
            .inner_join(results::table.on(races::race_id.eq(results::race_id)))
            .inner_join(drivers::table.on(results::driver_id.eq(drivers::driver_id)))
            .inner_join(
                constructors::table.on(results::constructor_id.eq(constructors::constructor_id)),
            )
            .inner_join(circuits::table.on(races::circuit_id.eq(circuits::circuit_id)))
            .select(Season::as_select())
            .distinct()
            .into_boxed()
    }

    fn filter(filter: SeasonFilter) -> Paginated<BoxedQuery> {
        let limit = filter.limit.unwrap_or_default().0 as i64;
        let page = filter.page.unwrap_or_default().0 as i64;

        let conditions = fields_to_filter!(
            filter,
            driver_ref => (DriverRef, StringFilter::Equal),
            constructor_ref => (ConstructorRef, StringFilter::Equal),
            circuit_ref => (CircuitRef, StringFilter::Equal),
            grid => (Grid, NumberFilter::Equal)
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
    ConstructorRef(StringFilter),
    CircuitRef(StringFilter),
    Grid(NumberFilter<i32>),
}

impl Condition {
    fn into_boxed_condition(self) -> Option<BoxedCondition> {
        use Condition::*;

        Some(match self {
            ConstructorRef(f) => string_filter!(f, constructors::constructor_ref),
            CircuitRef(f) => string_filter!(f, circuits::circuit_ref),
            DriverRef(f) => string_filter!(f, drivers::driver_ref),
            Grid(f) => number_filter!(f, results::grid),
        })
    }
}

mod types {
    use super::Season;
    use crate::prelude::*;
    use diesel::{
        helper_types::{
            AsSelect, Eq, InnerJoin, InnerJoinOn, InnerJoinQuerySource, IntoBoxed, Select,
        },
        prelude::*,
        sql_types::{Bool, Nullable},
    };

    pub type BoxedConditionSource = InnerJoinQuerySource<
        InnerJoinQuerySource<
            InnerJoinQuerySource<
                InnerJoinQuerySource<
                    InnerJoinQuerySource<seasons::table, races::table>,
                    results::table,
                    Eq<races::race_id, results::race_id>,
                >,
                drivers::table,
                Eq<results::driver_id, drivers::driver_id>,
            >,
            constructors::table,
            Eq<results::constructor_id, constructors::constructor_id>,
        >,
        circuits::table,
        Eq<races::circuit_id, circuits::circuit_id>,
    >;
    pub type BoxedCondition =
        Box<dyn BoxableExpression<BoxedConditionSource, crate::Backend, SqlType = Nullable<Bool>>>;

    pub type BoxedQuerySource = InnerJoinOn<
        InnerJoinOn<
            InnerJoinOn<
                InnerJoinOn<
                    InnerJoin<seasons::table, races::table>,
                    results::table,
                    Eq<races::race_id, results::race_id>,
                >,
                drivers::table,
                Eq<results::driver_id, drivers::driver_id>,
            >,
            constructors::table,
            Eq<results::constructor_id, constructors::constructor_id>,
        >,
        circuits::table,
        Eq<races::circuit_id, circuits::circuit_id>,
    >;
    pub type BoxedQuery = IntoBoxed<
        'static,
        Select<BoxedQuerySource, AsSelect<Season, crate::Backend>>,
        crate::Backend,
    >;
}
