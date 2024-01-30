use diesel::{Associations, Identifiable, Queryable, Selectable};

use crate::prelude::*;

#[derive(Identifiable, Queryable, Selectable, Debug)]
#[diesel(primary_key(year))]
#[diesel(table_name = seasons, check_for_backend(super::Backend))]
pub struct Season {
    pub year: i32,
    pub url: String,
}

#[derive(Identifiable, Queryable, Selectable, Debug)]
#[diesel(primary_key(constructor_id))]
#[diesel(table_name = constructors, check_for_backend(super::Backend))]
pub struct Constructor {
    pub constructor_id: i32,
    pub constructor_ref: String,
    pub name: String,
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, serde::Serialize)]
#[diesel(primary_key(race_id))]
#[diesel(belongs_to(Season, foreign_key = year))]
#[diesel(table_name = races, check_for_backend(super::Backend))]
pub struct Race {
    pub race_id: i32,
    pub year: i32,
    pub round: i32,
    pub circuit_id: i32,
    pub name: String,
    pub date: chrono::NaiveDate,
    pub time: Option<chrono::NaiveTime>,
    pub url: Option<String>,
    pub fp1_date: Option<chrono::NaiveDate>,
    pub fp1_time: Option<chrono::NaiveTime>,
    pub fp2_date: Option<chrono::NaiveDate>,
    pub fp2_time: Option<chrono::NaiveTime>,
    pub fp3_date: Option<chrono::NaiveDate>,
    pub fp3_time: Option<chrono::NaiveTime>,
    pub qualifying_date: Option<chrono::NaiveDate>,
    pub qualifying_time: Option<chrono::NaiveTime>,
    pub sprint_date: Option<chrono::NaiveDate>,
    pub sprint_time: Option<chrono::NaiveTime>,
}

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

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(primary_key(race_id))]
#[diesel(table_name = races, check_for_backend(super::Backend))]
pub struct RaceRoundAndYear {
    pub race_id: i32,
    pub year: i32,
    pub round: i32,
}

#[derive(Identifiable, Queryable, Selectable, Debug)]
#[diesel(primary_key(driver_standing_id))]
#[diesel(table_name = driverStandings, check_for_backend(super::Backend))]
pub struct DriverStanding {
    pub driver_standing_id: i32,
    pub points: f32,
    pub position: Option<i32>,
    pub position_text: Option<String>,
    pub wins: i32,
    #[diesel(embed)]
    pub driver: Driver,
    #[diesel(embed)]
    pub race_round_and_year: RaceRoundAndYear,
    #[diesel(embed)]
    pub constructor: Constructor,
}

impl From<Constructor> for shared::models::Constructor {
    fn from(constructor: Constructor) -> shared::models::Constructor {
        shared::models::Constructor {
            constructor_ref: constructor.constructor_ref,
            name: constructor.name,
            nationality: constructor.nationality,
            url: constructor.url,
        }
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

impl From<Season> for shared::models::Season {
    fn from(season: Season) -> shared::models::Season {
        shared::models::Season {
            year: season.year,
            url: season.url,
        }
    }
}

pub struct Tuple<A, B, C>(A, B, C);

impl<A, B, C> From<(A, B, C)> for Tuple<A, B, C> {
    fn from(value: (A, B, C)) -> Tuple<A, B, C> {
        Tuple(value.0, value.1, value.2)
    }
}

impl From<DriverStanding> for shared::models::DriverStanding {
    fn from(value: DriverStanding) -> shared::models::DriverStanding {
        shared::models::DriverStanding {
            year: value.race_round_and_year.year,
            round: value.race_round_and_year.round,
            driver: value.driver.into(),
            constructor: value.constructor.into(),
            points: value.points,
            wins: value.wins,
            position: value.position,
            position_text: value.position_text,
        }
    }
}
