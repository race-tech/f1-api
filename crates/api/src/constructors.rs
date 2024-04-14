use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/constructors?<constructor_ref>", rank = 1)]
fn constructors_ref(
    db: &State<ConnectionPool>,
    series: Series,
    constructor_ref: shared::parameters::ConstructorRef,
) -> Result<Json<Response<Constructor>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let constructor =
        application::constructors::ConstructorsQueryBuilder::get(constructor_ref, conn)?;

    let response = Response {
        data: constructor.into(),
        pagination: None,
        series,
    };

    Ok(Json(response))
}

#[get("/<series>/constructors?<param..>", rank = 2)]
fn constructors(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetConstructorsParameter,
) -> Result<Json<Response<Vec<Constructor>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let res =
        application::constructors::ConstructorsQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response::new(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![constructors, constructors_ref]
}
