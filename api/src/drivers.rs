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
    let (drivers, pagination) = driver_inner_handler(db, series, param.into())?;

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
    let mut filter: DriverFilter = param.into();
    filter.year = Some(year);

    let (drivers, pagination) = driver_inner_handler(db, series, filter)?;

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
    let mut filter: DriverFilter = param.into();
    filter.year = Some(year);
    filter.round = Some(round);

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
) -> Result<(Vec<Driver>, Pagination)> {
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
