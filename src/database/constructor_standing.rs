use diesel::dsl::{AsSelect, Filter, Select};
use diesel::prelude::*;
use diesel::{Identifiable, Queryable, Selectable};

use super::schema::constructorStandings;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(primary_key(constructor_standing_id))]
#[diesel(table_name = constructorStandings, check_for_backend(super::Backend))]
pub struct ConstructorStanding {
    pub constructor_standing_id: i32,
    pub race_id: i32,
    pub constructor_id: i32,
    pub points: f32,
    pub position: Option<i32>,
    pub position_text: Option<String>,
    pub wins: i32,
}

type All = Select<constructorStandings::table, AsSelect<ConstructorStanding, super::Backend>>;
type ByRaceId = Filter<All, diesel::dsl::Eq<constructorStandings::race_id, i32>>;
type ByConstructorId = Filter<All, diesel::dsl::Eq<constructorStandings::constructor_id, i32>>;

impl ConstructorStanding {
    pub fn all() -> All {
        constructorStandings::table.select(ConstructorStanding::as_select())
    }

    pub fn by_race_id(race_id: i32) -> ByRaceId {
        Self::all().filter(constructorStandings::race_id.eq(race_id))
    }

    pub fn by_constructor_id(constructor_id: i32) -> ByConstructorId {
        Self::all().filter(constructorStandings::constructor_id.eq(constructor_id))
    }
}
