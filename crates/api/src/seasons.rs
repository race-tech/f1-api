use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/seasons?<season>", rank = 1)]
pub fn seasons_year(
    db: &State<ConnectionPool>,
    series: Series,
    season: shared::parameters::Year,
) -> Result<Json<Response<Season>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let season = application::seasons::SeasonsQueryBuilder::get(season, conn)?;

    let response = Response {
        data: season.into(),
        pagination: None,
        series,
    };

    Ok(Json(response))
}

#[get("/<series>/seasons?<param..>", rank = 2)]
pub fn seasons(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetSeasonsParameters,
) -> Result<Json<Response<Vec<Season>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let res = application::seasons::SeasonsQueryBuilder::params(param).query_and_count(conn)?;

    let response = Response::new(res.0, res.1, series);

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![seasons, seasons_year]
}
