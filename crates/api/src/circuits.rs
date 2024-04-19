use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

use crate::guards::rate_limiter::RateLimiter;

#[get("/<series>/circuits?<circuit_ref>", rank = 1)]
fn circuits_ref(
    db: &State<ConnectionPool>,
    series: Series,
    circuit_ref: shared::parameters::CircuitRef,
    rate_limiter: RateLimiter,
) -> Result<Json<Response<Circuit>>> {
    let _ = rate_limiter;
    let conn = &mut db.from_series(series).get()?;

    let circuit = application::circuits::CircuitsQueryBuilder::get(circuit_ref, conn)?;

    let response = Response {
        data: circuit.into(),
        pagination: None,
        series,
    };

    Ok(Json(response))
}

#[get("/<series>/circuits?<param..>", rank = 2)]
fn circuits(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetCircuitsParameter,
    rate_limiter: RateLimiter,
) -> Result<Json<Response<Vec<Circuit>>>> {
    let _ = rate_limiter;
    let conn = &mut db.from_series(series).get()?;

    let res = application::circuits::CircuitsQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response::from_vec(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![circuits, circuits_ref]
}
