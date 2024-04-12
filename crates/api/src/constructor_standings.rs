use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/constructors/standing?<param..>")]
pub fn constructor_standings(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetConstructorStandingsParameter,
) -> Result<Json<Response<Vec<ConstructorStandings>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let query =
        application::constructor_standings::ConstructorStandingsQueryBuilder::params(param).build();

    let res = query.query_and_count(conn);

    let response = Response {
        data: res.0,
        pagination: res.1,
        series,
    };

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![constructor_standings]
}
