use rocket::serde::Serialize;

use super::models::{Constructor, Driver, Pagination, Series};

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
