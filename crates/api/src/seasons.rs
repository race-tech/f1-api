use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/seasons?<param..>")]
pub fn seasons(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetSeasonsParameters,
) -> Result<Json<Response<Vec<Season>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let query = application::seasons::SeasonsQueryBuilder::params(param).build();

    let res = query.query_and_count(conn);

    let response = Response::new(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![seasons]
}
