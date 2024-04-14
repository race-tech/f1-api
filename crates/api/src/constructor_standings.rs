use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/constructors/standing?<param..>")]
fn constructor_standings(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetConstructorStandingsParameter,
) -> Result<Json<Response<Vec<ConstructorStandingResponse>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let res = application::constructor_standings::ConstructorStandingsQueryBuilder::params(param)
        .query_and_count(conn)?;

    let response = Response::new(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![constructor_standings]
}
