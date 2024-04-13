use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/races?<param..>")]
pub fn races(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetRacesParameters,
) -> Result<Json<Response<Vec<RaceResponse>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let res = application::races::RacesQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response::new(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![races]
}
