use diesel::Connection;
use rocket::serde::json::Json;
use rocket::{get, routes, State};

use application;
use infrastructure::ConnectionPool;
use shared::prelude::*;

use crate::error::Result;

#[get(
    "/<series>/constructors?<limit>&<page>&<driver_number>&<driver_ref>&<constructor>&<circuit>&<grid>&<result>",
)]
pub fn constructors(
    db: &State<ConnectionPool>,
    series: Series,
    limit: Option<Limit>,
    page: Option<Page>,
    driver_number: Option<DriverNumber>,
    driver_ref: Option<DriverRef>,
    constructor: Option<ConstructorName>,
    circuit: Option<Circuit>,
    grid: Option<Grid>,
    result: Option<RaceResult>,
) -> Result<Json<ConstructorResponse>> {
    let filter = ConstructorFilter {
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

    let (constructors, pagination) = constructors_inner_handler(db, series, filter)?;

    let response = ConstructorResponse {
        pagination,
        series,
        constructors,
    };

    Ok(Json(response))
}

#[get(
    "/<series>/<year>/constructor?<limit>&<page>&<driver_number>&<driver_ref>&<constructor>&<circuit>&<grid>&<result>",
)]
pub fn constructors_by_year(
    db: &State<ConnectionPool>,
    series: Series,
    year: Year,
    limit: Option<Limit>,
    page: Option<Page>,
    driver_number: Option<DriverNumber>,
    driver_ref: Option<DriverRef>,
    constructor: Option<ConstructorName>,
    circuit: Option<Circuit>,
    grid: Option<Grid>,
    result: Option<RaceResult>,
) -> Result<Json<ConstructorResponse>> {
    let filter = ConstructorFilter {
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

    let (constructors, pagination) = constructors_inner_handler(db, series, filter)?;

    let response = ConstructorResponse {
        pagination,
        series,
        constructors,
    };

    Ok(Json(response))
}

#[get(
    "/<series>/<year>/<round>/constructors?<limit>&<page>&<driver_number>&<driver_ref>&<constructor>&<circuit>&<grid>&<result>",
)]
pub fn constructors_by_year_and_round(
    db: &State<ConnectionPool>,
    series: Series,
    year: Year,
    round: Round,
    limit: Option<Limit>,
    page: Option<Page>,
    driver_number: Option<DriverNumber>,
    driver_ref: Option<DriverRef>,
    constructor: Option<ConstructorName>,
    circuit: Option<Circuit>,
    grid: Option<Grid>,
    result: Option<RaceResult>,
) -> Result<Json<ConstructorResponse>> {
    let filter = ConstructorFilter {
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

    let (constructors, pagination) = constructors_inner_handler(db, series, filter)?;

    let response = ConstructorResponse {
        pagination,
        series,
        constructors,
    };

    Ok(Json(response))
}

fn constructors_inner_handler(
    db: &State<ConnectionPool>,
    series: Series,
    filter: ConstructorFilter,
) -> Result<(Vec<Constructor>, Pagination)> {
    let pool = &mut db.from_series(series).get()?;
    let res = pool.transaction(|conn| {
        application::models::Constructor::filter(filter).load_and_count_pages(conn)
    });

    Ok(res.map(|(constructors, pagination)| {
        (
            constructors
                .into_iter()
                .map(Constructor::from)
                .collect::<Vec<_>>(),
            pagination,
        )
    })?)
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![
        constructors,
        constructors_by_year,
        constructors_by_year_and_round
    ]
}
