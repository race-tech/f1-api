use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/drivers/standing?<param..>")]
fn driver_standings(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetDriverStandingsParameter,
) -> Result<Json<Response<Vec<DriverStandingResponse>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let res = application::driver_standings::DriverStandingsQueryBuilder::params(param)
        .query_and_count(conn)?;

    let response = Response::new(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![driver_standings]
}
