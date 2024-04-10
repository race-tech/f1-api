use diesel::{alias, prelude::*};
use shared::models::Pagination;

use crate::models::ConstructorStanding;
use crate::prelude::*;
use types::*;

alias!(races as r1: RaceAlias);

pub struct ConstructorStandingBuilder(shared::filters::ConstructorStandingFilter);

impl ConstructorStandingBuilder {
    pub fn new(filter: shared::filters::ConstructorStandingFilter) -> Self {
        Self(filter)
    }

    pub fn load(
        self,
        conn: &mut MysqlConnection,
    ) -> Result<(Vec<ConstructorStanding>, Pagination), diesel::result::Error> {
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
        } else if let Some(constructor_ref) = self.0.constructor_ref {
            Self::by_constructor_ref(constructor_ref.0)
                .paginate(page)
                .per_page(limit)
                .load_and_count_pages(conn)
        } else if let Some(result) = self.0.result {
            Self::by_result(result.0)
                .paginate(page)
                .per_page(limit)
                .load_and_count_pages(conn)
        } else {
            Self::all()
                .paginate(page)
                .per_page(limit)
                .load_and_count_pages(conn)
        }
    }

    fn all_left_join() -> All<CSLeftJoin> {
        races::table
            .left_join(
                r1.on(races::year
                    .eq(r1.field(races::year))
                    .and(races::round.lt(r1.field(races::round)))),
            )
            .inner_join(constructorStandings::table)
            .inner_join(
                constructors::table
                    .on(constructors::constructorId.eq(constructorStandings::constructorId)),
            )
            .select(ConstructorStanding::as_select())
    }

    fn default_join() -> All<CSDefaultJoin> {
        constructorStandings::table
            .inner_join(races::table)
            .inner_join(
                constructors::table
                    .on(constructorStandings::constructorId.eq(constructors::constructorId)),
            )
            .select(ConstructorStanding::as_select())
    }

    fn all() -> By<All<CSLeftJoin>, R1IdIsNull> {
        Self::all_left_join().filter(r1.field(races::raceId).is_null())
    }

    fn by_race_id(race_id: i32) -> By<All<CSDefaultJoin>, RaceId> {
        Self::default_join().filter(races::raceId.eq(race_id))
    }

    fn by_constructor_ref(constructor_ref: String) -> By<All<CSLeftJoin>, ConstructorRef> {
        Self::all_left_join().filter(
            constructors::constructorRef
                .eq(constructor_ref)
                .and(r1.field(races::raceId).is_null()),
        )
    }

    fn by_result(result: i32) -> By<All<CSLeftJoin>, CSResult> {
        Self::all_left_join().filter(
            constructorStandings::position
                .eq(result)
                .and(r1.field(races::raceId).is_null()),
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

    pub type All<T> = Select<T, AsSelect<super::ConstructorStanding, crate::Backend>>;
    pub type By<S, F> = Filter<S, F>;
    pub type CSLeftJoin = InnerJoinOn<
        InnerJoin<
            LeftJoinOn<
                races::table,
                Alias<RaceAlias>,
                And<
                    Eq<races::year, AliasedField<RaceAlias, races::year>>,
                    Lt<races::round, AliasedField<RaceAlias, races::round>>,
                >,
            >,
            constructorStandings::table,
        >,
        constructors::table,
        Eq<constructors::constructorId, constructorStandings::constructorId>,
    >;
    pub type CSDefaultJoin = InnerJoinOn<
        InnerJoin<constructorStandings::table, races::table>,
        constructors::table,
        Eq<constructorStandings::constructorId, constructors::constructorId>,
    >;

    macro_rules! operators {
        () => {};
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
    }

    operators! {
        _R1IdIsNull => alias @ { RaceAlias, races::raceId };
        R1IdIsNull => IsNull { _R1IdIsNull };
        RaceId => Eq { races::raceId, i32 };
        _ConstructorRef => Eq { constructors::constructorRef, String };
        _CSResult => Eq { constructorStandings::position, i32 };
        ConstructorRef => And { _ConstructorRef, R1IdIsNull };
        CSResult => And { _CSResult, R1IdIsNull };
    }
}
