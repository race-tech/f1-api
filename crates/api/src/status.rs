use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

use crate::guards::rate_limiter::RateLimiter;

#[get("/<series>/status?<param..>")]
fn status(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetStatusParameters,
    rate_limiter: RateLimiter,
) -> Result<Json<Response<Vec<Status>>>> {
    let _ = rate_limiter;
    let conn = &mut db.from_series(series).get()?;

    let res = application::status::StatusQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response::from_vec(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![status]
}
