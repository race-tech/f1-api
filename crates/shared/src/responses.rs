use rocket::serde::Serialize;

use super::prelude::*;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DriversResponse {
    pub drivers: Vec<Driver>,

    #[serde(flatten)]
    pub pagination: Pagination,
    pub series: Series,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ConstructorResponse {
    pub constructors: Vec<Constructor>,

    #[serde(flatten)]
    pub pagination: Pagination,
    pub series: Series,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StandingsResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub drivers_standings: Vec<DriverStanding>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub constructors_standings: Vec<ConstructorStanding>,
    #[serde(flatten)]
    pub pagination: Pagination,
    pub series: Series,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct SeasonsResponse {
    pub seasons: Vec<Season>,

    #[serde(flatten)]
    pub pagination: Pagination,
    pub series: Series,
}
