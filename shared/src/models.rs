use rocket::serde::Serialize;

use crate::parameters::*;

#[derive(Debug, Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Pagination {
    pub limit: i64,
    pub page: i64,
    pub max_page: i64,
    pub total: i64,
}

#[derive(Debug, Default, Serialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub enum Series {
    #[default]
    #[serde(rename = "f1")]
    F1,
    #[serde(rename = "f2")]
    F2,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Driver {
    pub driver_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "permanent_number")]
    pub number: Option<i32>,
    pub code: Option<String>,
    #[serde(rename = "given_name")]
    pub forename: String,
    #[serde(rename = "family_name")]
    pub surname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "date_of_birth")]
    pub dob: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(Debug)]
pub struct DriverFilter {
    pub limit: Option<Limit>,
    pub page: Option<Page>,
    pub driver_number: Option<DriverNumber>,
    pub driver_ref: Option<DriverRef>,
    pub constructor: Option<Constructor>,
    pub circuit: Option<Circuit>,
    pub grid: Option<Grid>,
    pub result: Option<RaceResult>,
}
