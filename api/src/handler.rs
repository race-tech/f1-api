use rocket::get;
use rocket::serde::json::Json;
use diesel::prelude::*;

use application;
use infrastructure;

#[get("/drivers")]
pub fn drivers() -> Json<Vec<application::models::Driver>> {
    let mut pool = infrastructure::Connection::default();
    let drivers: Vec<application::models::Driver> = application::models::Driver::limit(10).load(pool.pool()).unwrap();

    Json(drivers)
}
