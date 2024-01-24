use diesel::Connection;
use rocket::serde::json::Json;
use rocket::{get, routes, State};

use application;
use infrastructure::{self, ConnectionPool};
use shared::prelude::*;

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
) -> crate::error::Result<Json<DriversResponse>> {
    let filter = DriverFilter {
        limit,
        page,
        driver_ref,
        driver_number,
        constructor,
        circuit,
        grid,
        result,
        year: None,
        round: None,
    };

    let (drivers, pagination) = driver_inner_handler(db, series, filter)?;

    let response = DriversResponse {
        pagination,
        series,
        drivers,
    };

    Ok(Json(response))
}

#[get(
    "/<series>/drivers/<year>?<limit>&<page>&<driver_number>&<driver_ref>&<constructor>&<circuit>&<grid>&<result>",
)]
pub fn drivers_by_year(
    db: &rocket::State<infrastructure::ConnectionPool>,
    series: Series,
    year: Year,
    limit: Option<Limit>,
    page: Option<Page>,
    driver_number: Option<DriverNumber>,
    driver_ref: Option<DriverRef>,
    constructor: Option<Constructor>,
    circuit: Option<Circuit>,
    grid: Option<Grid>,
    result: Option<RaceResult>,
) -> crate::error::Result<Json<DriversResponse>> {
    let filter = DriverFilter {
        limit,
        page,
        driver_ref,
        driver_number,
        constructor,
        circuit,
        grid,
        result,
        year: Some(year),
        round: None,
    };

    let (drivers, pagination) = driver_inner_handler(db, series, filter)?;

    let response = DriversResponse {
        pagination,
        series,
        drivers,
    };

    Ok(Json(response))
}

#[get(
    "/<series>/drivers/<year>/<round>?<limit>&<page>&<driver_number>&<driver_ref>&<constructor>&<circuit>&<grid>&<result>",
)]
pub fn drivers_by_year_and_round(
    db: &rocket::State<infrastructure::ConnectionPool>,
    series: Series,
    year: Year,
    round: Round,
    limit: Option<Limit>,
    page: Option<Page>,
    driver_number: Option<DriverNumber>,
    driver_ref: Option<DriverRef>,
    constructor: Option<Constructor>,
    circuit: Option<Circuit>,
    grid: Option<Grid>,
    result: Option<RaceResult>,
) -> crate::error::Result<Json<DriversResponse>> {
    let filter = DriverFilter {
        limit,
        page,
        driver_ref,
        driver_number,
        constructor,
        circuit,
        grid,
        result,
        year: Some(year),
        round: Some(round),
    };

    let (drivers, pagination) = driver_inner_handler(db, series, filter)?;

    let response = DriversResponse {
        pagination,
        series,
        drivers,
    };

    Ok(Json(response))
}

fn driver_inner_handler(
    db: &State<ConnectionPool>,
    series: Series,
    filter: DriverFilter,
) -> crate::error::Result<(Vec<Driver>, Pagination)> {
    let pool = &mut db.from_series(series).get()?;
    let res = pool
        .transaction(|conn| application::models::Driver::filter(filter).load_and_count_pages(conn));

    Ok(res.map(|(drivers, pagination)| {
        (
            drivers.into_iter().map(Driver::from).collect::<Vec<_>>(),
            pagination,
        )
    })?)
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![drivers, drivers_by_year, drivers_by_year_and_round]
}
