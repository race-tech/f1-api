use diesel::Connection;
use rocket::serde::json::Json;
use rocket::{get, routes};

use application;
use infrastructure;
use shared::models::{Driver, DriverFilter};
use shared::parameters::{
    Circuit, Constructor, DriverNumber, DriverRef, Grid, Limit, Page, RaceResult, Series,
};
use shared::responses::DriversResponse;

#[get(
    "/<series>/drivers?<limit>&<page>&<driver_number>&<driver_ref>&<constructor>&<circuit>&<grid>&<result>",
)]
pub fn drivers(
    db: &rocket::State<infrastructure::ConnectionPool>,
    series: Series,
    limit: Option<Limit>,
    page: Option<Page>,
    driver_number: Option<DriverNumber>,
    driver_ref: Option<DriverRef>,
    constructor: Option<Constructor>,
    circuit: Option<Circuit>,
    grid: Option<Grid>,
    result: Option<RaceResult>,
) -> Json<DriversResponse> {
    let filter = DriverFilter {
        limit,
        page,
        driver_ref,
        driver_number,
        constructor,
        circuit,
        grid,
        result,
    };

    let pool = &mut db.from_series(series).get().unwrap();
    let res = pool
        .transaction(|conn| application::models::Driver::filter(filter).load_and_count_pages(conn));

    let (drivers, pagination) = match res {
        Ok(res) => res,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };
    let drivers = drivers.into_iter().map(Driver::from).collect::<Vec<_>>();

    let response = DriversResponse {
        pagination,
        series,
        drivers,
    };

    Json(response)
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![drivers]
}
