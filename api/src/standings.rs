use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{get, routes, State};

use application;
use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/standings/drivers?<param..>", rank = 1)]
pub fn drivers_standing(
    db: &State<ConnectionPool>,
    series: Series,
    param: DriverStandingParameter,
) -> Result<Json<StandingsResponse>> {
    let response = driver_standing_inner_handler(db, series, None, None, param)?;

    Ok(Json(response))
}

#[get("/<series>/<year>/standings/drivers?<param..>", rank = 1)]
pub fn drivers_standing_with_year(
    db: &State<ConnectionPool>,
    series: Series,
    param: DriverStandingParameter,
    year: Year,
) -> Result<Json<StandingsResponse>> {
    let response = driver_standing_inner_handler(db, series, Some(year), None, param)?;

    Ok(Json(response))
}

#[get("/<series>/<year>/<round>/standings/drivers?<param..>", rank = 1)]
pub fn drivers_standing_with_year_and_round(
    db: &State<ConnectionPool>,
    series: Series,
    year: Year,
    round: Round,
    param: DriverStandingParameter,
) -> Result<Json<StandingsResponse>> {
    let response = driver_standing_inner_handler(db, series, Some(year), Some(round), param)?;

    Ok(Json(response))
}

#[get("/<series>/standings/constructors?<param..>", rank = 1)]
pub fn constructors_standing(
    db: &State<ConnectionPool>,
    series: Series,
    param: ConstructorStandingParameter,
) -> Result<Json<StandingsResponse>> {
    let response = constructors_standing_inner_handler(db, series, None, None, param)?;

    Ok(Json(response))
}

#[get("/<series>/<year>/standings/constructors?<param..>", rank = 1)]
pub fn constructors_standing_with_year(
    db: &State<ConnectionPool>,
    series: Series,
    param: ConstructorStandingParameter,
    year: Year,
) -> Result<Json<StandingsResponse>> {
    let response = constructors_standing_inner_handler(db, series, Some(year), None, param)?;

    Ok(Json(response))
}

#[get("/<series>/<year>/<round>/constructors/drivers?<param..>", rank = 1)]
pub fn constructors_standing_with_year_and_round(
    db: &State<ConnectionPool>,
    series: Series,
    year: Year,
    round: Round,
    param: ConstructorStandingParameter,
) -> Result<Json<StandingsResponse>> {
    let response = constructors_standing_inner_handler(db, series, Some(year), Some(round), param)?;

    Ok(Json(response))
}

fn driver_standing_inner_handler(
    db: &State<ConnectionPool>,
    series: Series,
    year: Option<Year>,
    round: Option<Round>,
    param: DriverStandingParameter,
) -> Result<StandingsResponse> {
    let mut filter: DriverStandingFilter = param.into();
    filter.year = year;
    filter.round = round;

    filter.validate()?;

    let pool = &mut db.from_series(series).get()?;
    let (drivers_standings, pagination) = pool.transaction(|conn| {
        let (vec, pagination) =
            application::builders::DriverStandingBuilder::new(filter).load(conn)?;

        Ok::<_, Error>((vec, pagination))
    })?;
    let (season, round) = if let Some(f) = drivers_standings.first() {
        (
            Some(f.race_round_and_year.year),
            Some(f.race_round_and_year.round),
        )
    } else {
        (None, None)
    };
    let drivers_standings = drivers_standings
        .into_iter()
        .map(|s| s.into())
        .collect::<Vec<_>>();

    Ok(StandingsResponse {
        season,
        round,
        drivers_standings,
        constructors_standings: Vec::new(),
        pagination,
        series,
    })
}

fn constructors_standing_inner_handler(
    db: &State<ConnectionPool>,
    series: Series,
    year: Option<Year>,
    round: Option<Round>,
    param: ConstructorStandingParameter,
) -> Result<StandingsResponse> {
    let mut filter: ConstructorStandingFilter = param.into();
    filter.year = year;
    filter.round = round;

    filter.validate()?;

    let pool = &mut db.from_series(series).get()?;
    let (constructors_standings, pagination) = pool.transaction(|conn| {
        let (vec, pagination) =
            application::builders::ConstructorStandingBuilder::new(filter).load(conn)?;

        Ok::<_, Error>((vec, pagination))
    })?;
    let (season, round) = if let Some(f) = constructors_standings.first() {
        (
            Some(f.race_round_and_year.year),
            Some(f.race_round_and_year.round),
        )
    } else {
        (None, None)
    };
    let constructors_standings = constructors_standings
        .into_iter()
        .map(|s| s.into())
        .collect::<Vec<_>>();

    Ok(StandingsResponse {
        season,
        round,
        drivers_standings: Vec::new(),
        constructors_standings,
        pagination,
        series,
    })
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![
        drivers_standing,
        drivers_standing_with_year,
        drivers_standing_with_year_and_round,
        constructors_standing,
        constructors_standing_with_year,
        constructors_standing_with_year_and_round
    ]
}
