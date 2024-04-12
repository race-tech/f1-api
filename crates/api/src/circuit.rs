use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/circuits?<param..>")]
pub fn circuits(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetCircuitsParameter,
) -> Result<Json<Response<Vec<Circuits>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let query = application::circuit::CircuitQueryBuilder::params(param).build();

    let res = query.query_and_count(conn);

    let response = Response {
        data: res.0,
        pagination: res.1,
        series,
    };

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![circuits]
}
