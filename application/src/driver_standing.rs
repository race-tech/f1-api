use diesel::dsl::{AsSelect, Filter, Select};
use diesel::prelude::*;
use diesel::{Identifiable, Queryable, Selectable};

use super::schema::driverStandings;

#[derive(Queryable, Selectable, Identifiable, Debug, serde::Serialize)]
#[diesel(primary_key(driver_standing_id))]
#[diesel(table_name = driverStandings, check_for_backend(super::Backend))]
pub struct DriverStanding {
    pub driver_standing_id: i32,
    pub race_id: i32,
    pub driver_id: i32,
    pub points: f32,
    pub position: Option<i32>,
    pub position_text: Option<String>,
    pub wins: i32,
}

type All = Select<driverStandings::table, AsSelect<DriverStanding, super::Backend>>;
type ByRaceId = Filter<All, diesel::dsl::Eq<driverStandings::race_id, i32>>;
type ByDriverId = Filter<All, diesel::dsl::Eq<driverStandings::driver_id, i32>>;

impl DriverStanding {
    pub fn all() -> All {
        driverStandings::table.select(DriverStanding::as_select())
    }

    pub fn by_race_id(race_id: i32) -> ByRaceId {
        Self::all().filter(driverStandings::race_id.eq(race_id))
    }

    pub fn by_driver_id(driver_id: i32) -> ByDriverId {
        Self::all().filter(driverStandings::driver_id.eq(driver_id))
    }
}
