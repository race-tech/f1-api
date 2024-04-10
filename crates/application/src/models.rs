use diesel::{Identifiable, Queryable, Selectable};

use crate::prelude::*;

#[derive(Identifiable, Queryable, Selectable, Debug)]
#[diesel(primary_key(year))]
#[diesel(table_name = seasons, check_for_backend(super::Backend))]
pub struct Season {
    pub year: i32,
    pub url: String,
}

#[derive(Identifiable, Queryable, Selectable, Debug)]
#[diesel(primary_key(constructorId))]
#[diesel(table_name = constructors, check_for_backend(super::Backend))]
pub struct Constructor {
    #[diesel(column_name = "constructorId")]
    pub constructor_id: i32,
    #[diesel(column_name = "constructorRef")]
    pub constructor_ref: String,
    pub name: String,
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(primary_key(raceId))]
#[diesel(table_name = races, check_for_backend(super::Backend))]
pub struct Race {
    #[diesel(column_name = "raceId")]
    pub race_id: i32,
    pub year: i32,
    pub round: i32,
    #[diesel(column_name = "circuitId")]
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
    #[diesel(column_name = "quali_date")]
    pub qualifying_date: Option<chrono::NaiveDate>,
    #[diesel(column_name = "quali_time")]
    pub qualifying_time: Option<chrono::NaiveTime>,
    pub sprint_date: Option<chrono::NaiveDate>,
    pub sprint_time: Option<chrono::NaiveTime>,
}

#[derive(Identifiable, Queryable, Selectable, Debug)]
#[diesel(primary_key(driverId))]
#[diesel(table_name = drivers, check_for_backend(super::Backend))]
pub struct Driver {
    #[diesel(column_name = "driverId")]
    pub driver_id: i32,
    #[diesel(column_name = "driverRef")]
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
#[diesel(primary_key(raceId))]
#[diesel(table_name = races, check_for_backend(super::Backend))]
pub struct RaceRoundAndYear {
    #[diesel(column_name = "raceId")]
    pub race_id: i32,
    pub year: i32,
    pub round: i32,
}

#[derive(Identifiable, Queryable, Selectable, Debug)]
#[diesel(primary_key(driverStandingsId))]
#[diesel(table_name = driverStandings, check_for_backend(super::Backend))]
pub struct DriverStanding {
    #[diesel(column_name = "driverStandingsId")]
    pub driver_standing_id: i32,
    pub points: f32,
    pub position: Option<i32>,
    #[diesel(column_name = "positionText")]
    pub position_text: Option<String>,
    pub wins: i32,
    #[diesel(embed)]
    pub driver: Driver,
    #[diesel(embed)]
    pub race_round_and_year: RaceRoundAndYear,
}

#[derive(Identifiable, Queryable, Selectable, Debug)]
#[diesel(primary_key(constructorStandingsId))]
#[diesel(table_name = constructorStandings, check_for_backend(super::Backend))]
pub struct ConstructorStanding {
    #[diesel(column_name = "constructorStandingsId")]
    pub constructor_standing_id: i32,
    pub points: f32,
    pub position: Option<i32>,
    #[diesel(column_name = "positionText")]
    pub position_text: Option<String>,
    pub wins: i32,
    #[diesel(embed)]
    pub constructor: Constructor,
    #[diesel(embed)]
    pub race_round_and_year: RaceRoundAndYear,
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

impl From<DriverStanding> for shared::models::DriverStanding {
    fn from(value: DriverStanding) -> shared::models::DriverStanding {
        shared::models::DriverStanding {
            driver: value.driver.into(),
            points: value.points,
            wins: value.wins,
            position: value.position,
            position_text: value.position_text,
        }
    }
}

impl From<ConstructorStanding> for shared::models::ConstructorStanding {
    fn from(value: ConstructorStanding) -> shared::models::ConstructorStanding {
        shared::models::ConstructorStanding {
            constructor: value.constructor.into(),
            points: value.points,
            wins: value.wins,
            position: value.position,
            position_text: value.position_text,
            season: value.race_round_and_year.year,
            round: value.race_round_and_year.round,
        }
    }
}
