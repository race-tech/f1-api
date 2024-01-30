use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use rocket::serde::json::Json;
use rocket::{get, routes, State};

use application::{self, models};
use infrastructure::ConnectionPool;
use shared::prelude::*;

use crate::error::Result;

#[get("/<series>/standings/drivers?<param..>", rank = 1)]
pub fn standing(
    db: &State<ConnectionPool>,
    series: Series,
    param: DriverStandingParameter,
) -> Result<Json<StandingsResponse>> {
    let response = driver_standing_inner_handler(db, series, None, None, param)?;

    Ok(Json(response))
}

#[get("/<series>/<year>/standings/drivers?<param..>", rank = 1)]
pub fn standing_with_year(
    db: &State<ConnectionPool>,
    series: Series,
    param: DriverStandingParameter,
    year: Year,
) -> Result<Json<StandingsResponse>> {
    let response = driver_standing_inner_handler(db, series, Some(year), None, param)?;

    Ok(Json(response))
}

#[get("/<series>/<year>/<round>/standings/drivers?<param..>", rank = 1)]
pub fn standing_with_year_and_round(
    db: &State<ConnectionPool>,
    series: Series,
    param: DriverStandingParameter,
    year: Year,
    round: Round,
) -> Result<Json<StandingsResponse>> {
    let response = driver_standing_inner_handler(db, series, Some(year), Some(round), param)?;

    Ok(Json(response))
}

fn driver_standing_inner_handler(
    db: &State<ConnectionPool>,
    series: Series,
    year: Option<Year>,
    round: Option<Round>,
    param: DriverStandingParameter,
) -> Result<StandingsResponse> {
    let f = |conn: &mut PooledConnection<ConnectionManager<MysqlConnection>>| {
        let filter: DriverStandingFilter = if let Some(year) = year {
            let race = if let Some(round) = round {
                models::Race::by_year_and_round(year, round).first(conn)?
            } else {
                models::Race::last_race_of_year(year).first(conn)?
            };
            let mut filter: DriverStandingFilter = param.into();
            filter.race_id = Some(race.race_id.into());
            filter
        } else {
            param.into()
        };

        let (vec, pagination) =
            application::builders::DriverStandingBuilder::new(filter).load(conn)?;

        Ok::<_, crate::error::Error>((vec, pagination))
    };

    let pool = &mut db.from_series(series).get()?;
    let (drivers_standings, pagination) = pool.transaction(f)?;
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

pub fn handlers() -> Vec<rocket::Route> {
    routes![standing, standing_with_year, standing_with_year_and_round]
}
