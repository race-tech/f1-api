use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

use crate::guards::rate_limiter::RateLimiter;

#[get("/<series>/circuits?<circuit_ref>", rank = 1)]
pub fn circuits_ref(
    db: &State<ConnectionPool>,
    series: Series,
    circuit_ref: shared::parameters::CircuitRef,
    __rate_limiter: RateLimiter,
) -> Result<Json<Response<Circuit>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let circuit = application::circuits::CircuitsQueryBuilder::get(circuit_ref, conn)?;

    let response = Response {
        data: circuit.into(),
        pagination: None,
        series,
    };

    Ok(Json(response))
}

#[get("/<series>/circuits?<param..>", rank = 2)]
pub fn circuits(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetCircuitsParameter,
    __rate_limiter: &State<RateLimiter>,
) -> Result<Json<Response<Vec<Circuit>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let res = application::circuits::CircuitsQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response::new(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![circuits, circuits_ref]
}
