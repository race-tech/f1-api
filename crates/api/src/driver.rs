use rocket::serde::json::Json;
use rocket::{get, routes, State};

use application::{self, models::Drivers};
use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/drivers?<param..>")]
pub fn driver(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetDriversParameter,
) -> Result<Json<Response<Vec<Drivers>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let query = application::driver::DriverQueryBuilder::filter(param.into()).build();

    let res = query.query_and_count::<Drivers>(conn);

    let response = Response {
        data: res.0,
        pagination: res.1,
        series,
    };

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![driver]
}