use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/drivers/standing?<param..>")]
pub fn driver_standings(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetDriverStandingsParameter,
) -> Result<Json<Response<Vec<DriverStandings>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let query = application::driver_standings::DriverStandingsQueryBuilder::params(param).build();

    let res = query.query_and_count(conn);

    let response = Response {
        data: res.0,
        pagination: res.1,
        series,
    };

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![driver_standings]
}
