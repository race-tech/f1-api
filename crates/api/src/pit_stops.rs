use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

use crate::guards::rate_limiter::RateLimiter;

#[get("/<series>/pit_stops?<param..>")]
fn pit_stops(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetPitStopsParameter,
    rate_limiter: RateLimiter,
) -> Result<Json<Response<PitStopsResponse>>> {
    let _ = rate_limiter;
    let conn = &mut db.from_series(series).get()?;

    let res = application::pit_stops::PitStopsQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response {
        data: res.0.try_into()?,
        pagination: Some(res.1),
        series,
    };

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![pit_stops]
}
