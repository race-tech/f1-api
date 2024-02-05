use diesel::Connection;
use rocket::serde::json::Json;
use rocket::{get, routes, State};

use application;
use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/drivers?<param..>")]
pub fn drivers(
    db: &State<ConnectionPool>,
    series: Series,
    param: DriverParameter,
) -> Result<Json<DriversResponse>> {
    let (drivers, pagination) = driver_inner_handler(db, series, None, None, param)?;

    let response = DriversResponse {
        pagination,
        series,
        drivers,
    };

    Ok(Json(response))
}

#[get("/<series>/<year>/drivers?<param..>")]
pub fn drivers_by_year(
    db: &State<ConnectionPool>,
    series: Series,
    year: Year,
    param: DriverParameter,
) -> Result<Json<DriversResponse>> {
    let (drivers, pagination) = driver_inner_handler(db, series, Some(year), None, param)?;

    let response = DriversResponse {
        pagination,
        series,
        drivers,
    };

    Ok(Json(response))
}

#[get("/<series>/<year>/<round>/drivers?<param..>")]
pub fn drivers_by_year_and_round(
    db: &State<ConnectionPool>,
    series: Series,
    year: Year,
    round: Round,
    param: DriverParameter,
) -> Result<Json<DriversResponse>> {
    let (drivers, pagination) = driver_inner_handler(db, series, Some(year), Some(round), param)?;

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
    year: Option<Year>,
    round: Option<Round>,
    filter: DriverParameter,
) -> Result<(Vec<Driver>, Pagination)> {
    let mut filter: DriverFilter = filter.into();
    filter.year = year;
    filter.round = round;

    filter.validate()?;

    let pool = &mut db.from_series(series).get()?;
    let res = pool.transaction(|conn| application::builders::DriverBuilder::new(filter).load(conn));

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
