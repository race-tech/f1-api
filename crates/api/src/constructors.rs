use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/constructors?<param..>")]
pub fn constructors(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetConstructorsParameter,
) -> Result<Json<Response<Vec<Constructor>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let query = application::constructors::ConstructorsQueryBuilder::params(param).build();

    let res = query.query_and_count(conn);

    let response = Response::new(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![constructors]
}
