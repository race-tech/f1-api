use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

use crate::guards::rate_limiter::RateLimiter;

#[get("/<series>/races?<param..>")]
fn races(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetRacesParameters,
    rate_limiter: RateLimiter,
) -> Result<Json<Response<Vec<RaceResponse>>>> {
    let _ = rate_limiter;
    let conn = &mut db.from_series(series).get()?;

    let res = application::races::RacesQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response::from_vec(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![races]
}
