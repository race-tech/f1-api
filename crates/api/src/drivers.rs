use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/drivers?<driver_ref>", rank = 1)]
pub fn drivers_ref(
    db: &State<ConnectionPool>,
    series: Series,
    driver_ref: shared::parameters::DriverRef,
) -> Result<Json<Response<Driver>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let driver = application::drivers::DriversQueryBuilder::get(driver_ref, conn);

    let response = Response {
        data: driver.into(),
        pagination: None,
        series,
    };

    Ok(Json(response))
}

#[get("/<series>/drivers?<param..>", rank = 2)]
pub fn drivers(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetDriversParameter,
) -> Result<Json<Response<Vec<Driver>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let query = application::drivers::DriversQueryBuilder::params(param).build();

    let res = query.query_and_count(conn);

    let response = Response::new(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![drivers, drivers_ref]
}
