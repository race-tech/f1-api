use rocket::serde::Serialize;

use super::models::{Driver, Pagination, Series};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DriversResponse {
    pub drivers: Vec<Driver>,

    #[serde(flatten)]
    pub pagination: Pagination,
    pub series: Series,
}
