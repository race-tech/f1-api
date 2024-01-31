use diesel::Connection;
use rocket::serde::json::Json;
use rocket::{get, routes, State};

use application;
use infrastructure::ConnectionPool;
use shared::prelude::*;

use crate::error::Result;

#[get("/<series>/constructors?<param..>")]
pub fn constructors(
    db: &State<ConnectionPool>,
    series: Series,
    param: ConstructorParameter,
) -> Result<Json<ConstructorResponse>> {
    let (constructors, pagination) = constructors_inner_handler(db, series, param.into())?;

    let response = ConstructorResponse {
        pagination,
        series,
        constructors,
    };

    Ok(Json(response))
}

#[get("/<series>/<year>/constructors?<param..>")]
pub fn constructors_by_year(
    db: &State<ConnectionPool>,
    series: Series,
    year: Year,
    param: ConstructorParameter,
) -> Result<Json<ConstructorResponse>> {
    let mut filter: ConstructorFilter = param.into();
    filter.year = Some(year);

    let (constructors, pagination) = constructors_inner_handler(db, series, filter)?;

    let response = ConstructorResponse {
        pagination,
        series,
        constructors,
    };

    Ok(Json(response))
}

#[get("/<series>/<year>/<round>/constructors?<param..>")]
pub fn constructors_by_year_and_round(
    db: &State<ConnectionPool>,
    series: Series,
    year: Year,
    round: Round,
    param: ConstructorParameter,
) -> Result<Json<ConstructorResponse>> {
    let mut filter: ConstructorFilter = param.into();
    filter.year = Some(year);
    filter.round = Some(round);

    let (constructors, pagination) = constructors_inner_handler(db, series, filter)?;

    let response = ConstructorResponse {
        pagination,
        series,
        constructors,
    };

    Ok(Json(response))
}

fn constructors_inner_handler(
    db: &State<ConnectionPool>,
    series: Series,
    filter: ConstructorFilter,
) -> Result<(Vec<Constructor>, Pagination)> {
    let pool = &mut db.from_series(series).get()?;
    let res =
        pool.transaction(|conn| application::builders::ConstructorBuilder::new(filter).load(conn));

    Ok(res.map(|(constructors, pagination)| {
        (
            constructors
                .into_iter()
                .map(Constructor::from)
                .collect::<Vec<_>>(),
            pagination,
        )
    })?)
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![
        constructors,
        constructors_by_year,
        constructors_by_year_and_round
    ]
}
