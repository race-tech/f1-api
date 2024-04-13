use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/status?<status_id>", rank = 1)]
pub fn status_id(
    db: &State<ConnectionPool>,
    series: Series,
    status_id: shared::parameters::StatusId,
) -> Result<Json<Response<Status>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let status = application::status::StatusQueryBuilder::get(status_id, conn);

    let response = Response {
        data: status.into(),
        pagination: None,
        series,
    };

    Ok(Json(response))
}

#[get("/<series>/status?<param..>", rank = 2)]
pub fn status(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetStatusParameters,
) -> Result<Json<Response<Vec<Status>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let query = application::status::StatusQueryBuilder::params(param).build();

    let res = query.query_and_count(conn);

    let response = Response::new(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![status, status_id]
}
