use diesel::Connection;
use rocket::serde::json::Json;
use rocket::{get, routes, Route, State};

use application;
use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/seasons?<param..>")]
pub fn seasons(
    db: &State<ConnectionPool>,
    series: Series,
    param: SeasonParameter,
) -> Result<Json<SeasonsResponse>> {
    let (seasons, pagination) = season_inner_handler(db, series, param)?;

    let response = SeasonsResponse {
        pagination,
        series,
        seasons,
    };

    Ok(Json(response))
}

fn season_inner_handler(
    db: &State<ConnectionPool>,
    series: Series,
    param: SeasonParameter,
) -> Result<(Vec<Season>, Pagination)> {
    let filter: SeasonFilter = param.into();
    filter.validate()?;

    let pool = &mut db.from_series(series).get()?;
    let res = pool.transaction(|conn| application::builders::SeasonBuilder::new(filter).load(conn));

    Ok(res.map(|(seasons, pagination)| {
        (
            seasons.into_iter().map(Season::from).collect::<Vec<_>>(),
            pagination,
        )
    })?)
}

pub fn handlers() -> Vec<Route> {
    routes![seasons]
}
