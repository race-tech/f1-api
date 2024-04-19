use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

use crate::guards::rate_limiter::RateLimiter;

#[get("/<series>/seasons?<season>", rank = 1)]
fn seasons_year(
    db: &State<ConnectionPool>,
    series: Series,
    season: shared::parameters::Year,
    rate_limiter: RateLimiter,
) -> Result<Json<Response<Season>>> {
    let _ = rate_limiter;
    let conn = &mut db.from_series(series).get()?;

    let season = application::seasons::SeasonsQueryBuilder::get(season, conn)?;

    let response = Response {
        data: season.into(),
        pagination: None,
        series,
    };

    Ok(Json(response))
}

#[get("/<series>/seasons?<param..>", rank = 2)]
fn seasons(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetSeasonsParameters,
    rate_limiter: RateLimiter,
) -> Result<Json<Response<Vec<Season>>>> {
    let _ = rate_limiter;
    let conn = &mut db.from_series(series).get()?;

    let res = application::seasons::SeasonsQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response::from_vec(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![seasons, seasons_year]
}
