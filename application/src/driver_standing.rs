use diesel::{alias, prelude::*};
use shared::models::Pagination;

use crate::models::DriverStanding;
use crate::prelude::*;
use types::*;

alias!(races as r1: RaceAlias);

pub struct DriverStandingBuilder(shared::filters::DriverStandingFilter);

impl DriverStandingBuilder {
    pub fn new(filter: shared::filters::DriverStandingFilter) -> Self {
        Self(filter)
    }

    pub fn load(
        self,
        conn: &mut MysqlConnection,
    ) -> Result<(Vec<DriverStanding>, Pagination), diesel::result::Error> {
        let page = self.0.page.unwrap_or_default().0 as _;
        let limit = self.0.limit.unwrap_or_default().0 as _;

        if let Some(year) = self.0.year {
            let race = if let Some(round) = self.0.round {
                models::Race::by_year_and_round(year, round).first(conn)?
            } else {
                models::Race::last_race_of_year(year).first(conn)?
            };

            Self::by_race_id(race.race_id)
                .paginate(page)
                .per_page(limit)
                .load_and_count_pages(conn)
        } else if let Some(driver_ref) = self.0.name {
            Self::by_driver_ref(driver_ref.0)
                .paginate(page)
                .per_page(limit)
                .load_and_count_pages(conn)
        } else {
            Self::by_result(self.0.result.unwrap().0)
                .paginate(page)
                .per_page(limit)
                .load_and_count_pages(conn)
        }
    }

    fn all_left_join() -> All<DSLeftJoin> {
        races::table
            .left_join(
                r1.on(races::year
                    .eq(r1.field(races::year))
                    .and(races::round.lt(r1.field(races::round)))),
            )
            .inner_join(driverStandings::table)
            .inner_join(drivers::table.on(drivers::driver_id.eq(driverStandings::driver_id)))
            .select(DriverStanding::as_select())
    }

    fn default_join() -> All<DSDefaultJoin> {
        driverStandings::table
            .inner_join(races::table)
            .inner_join(drivers::table.on(driverStandings::driver_id.eq(drivers::driver_id)))
            .select(DriverStanding::as_select())
    }

    fn by_race_id(race_id: i32) -> By<All<DSDefaultJoin>, RaceId> {
        Self::default_join().filter(races::race_id.eq(race_id))
    }

    fn by_driver_ref(driver_ref: String) -> By<All<DSLeftJoin>, DriverRef> {
        Self::all_left_join().filter(
            drivers::driver_ref
                .eq(driver_ref)
                .and(r1.field(races::race_id).is_null()),
        )
    }

    fn by_result(result: i32) -> By<All<DSLeftJoin>, DSResult> {
        Self::all_left_join().filter(
            driverStandings::position
                .eq(result)
                .and(r1.field(races::race_id).is_null()),
        )
    }
}

mod types {
    use diesel::{
        helper_types::{And, AsSelect, Eq, Filter, InnerJoin, InnerJoinOn, LeftJoinOn, Lt, Select},
        query_source::{Alias, AliasedField},
    };

    use super::RaceAlias;
    use crate::schema::*;

    pub type All<T> = Select<T, AsSelect<super::DriverStanding, crate::Backend>>;
    pub type By<S, F> = Filter<S, F>;
    pub type DSLeftJoin = InnerJoinOn<
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
    >;
    pub type DSDefaultJoin = InnerJoinOn<
        InnerJoin<driverStandings::table, races::table>,
        drivers::table,
        Eq<driverStandings::driver_id, drivers::driver_id>,
    >;

    macro_rules! operators {
        ($name:ident => $op:ident { $column:path, $type:ty }; $($rest:tt)*) => {
            pub type $name = diesel::helper_types::$op < $column, $type >;
            operators! { $($rest)* }
        };
        ($name:ident => $op:ident { $type:ty }; $($rest:tt)*) => {
            pub type $name = diesel::helper_types::$op < $type >;
            operators! { $($rest)* }
        };
        ($name:ident => alias @ { $alias:ident, $column:path }; $($rest:tt)*) => {
            pub type $name = diesel::query_source::AliasedField < $alias, $column >;
            operators! { $($rest)* }
        };
        () => {}
    }

    operators! {
        _R1IdIsNull => alias @ { RaceAlias, races::race_id };
        R1IdIsNull => IsNull { _R1IdIsNull };
        RaceId => Eq { races::race_id, i32 };
        _DriverRef => Eq { drivers::driver_ref, String };
        _DSResult => Eq { driverStandings::position, i32 };
        DriverRef => And { _DriverRef, R1IdIsNull };
        DSResult => And { _DSResult, R1IdIsNull };
    }
}
