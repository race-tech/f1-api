use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/pit_stops?<param..>")]
fn pit_stops(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetPitStopsParameter,
) -> Result<Json<Response<PitStopsResponse>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let res = application::pit_stops::PitStopsQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response {
        data: res.0.into(),
        pagination: Some(res.1),
        series,
    };

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![pit_stops]
}
