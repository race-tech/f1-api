use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

use crate::guards::rate_limiter::RateLimiter;

#[get("/<series>/constructors/standing?<param..>")]
fn constructor_standings(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetConstructorStandingsParameter,
    rate_limiter: RateLimiter,
) -> Result<Json<Response<ConstructorStandingResponse>>> {
    let _ = rate_limiter;
    let conn = &mut db.from_series(series).get()?;

    let res = application::constructor_standings::ConstructorStandingsQueryBuilder::params(param)
        .query_and_count(conn)?;

    let response = Response {
        data: res.0.into(),
        pagination: Some(res.1),
        series,
    };

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![constructor_standings]
}
