use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

use crate::guards::rate_limiter::RateLimiter;

#[get("/<series>/drivers?<driver_ref>", rank = 1)]
fn drivers_ref(
    db: &State<ConnectionPool>,
    series: Series,
    driver_ref: shared::parameters::DriverRef,
    rate_limiter: RateLimiter,
) -> Result<Json<Response<Driver>>> {
    let _ = rate_limiter;
    let conn = &mut db.from_series(series).get()?;

    let driver = application::drivers::DriversQueryBuilder::get(driver_ref, conn)?;

    let response = Response {
        data: driver.into(),
        pagination: None,
        series,
    };

    Ok(Json(response))
}

#[get("/<series>/drivers?<param..>", rank = 2)]
fn drivers(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetDriversParameter,
    rate_limiter: RateLimiter,
) -> Result<Json<Response<Vec<Driver>>>> {
    let _ = rate_limiter;
    let conn = &mut db.from_series(series).get()?;

    let res = application::drivers::DriversQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response::from_vec(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![drivers, drivers_ref]
}
