use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/status?<param..>")]
fn status(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetStatusParameters,
) -> Result<Json<Response<Vec<Status>>>> {
    let conn = &mut db.from_series(series).get()?;

    let res = application::status::StatusQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response::new(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![status]
}
