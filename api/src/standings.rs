use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::Connection;
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
    let (drivers_standings, pagination) = driver_standing_inner_handler(db, series, None, param)?;

    let response = StandingsResponse {
        pagination,
        series,
        drivers_standings,
        constructors_standings: Vec::new(),
    };

    Ok(Json(response))
}

#[get("/<series>/<year>/standings/drivers?<param..>", rank = 1)]
pub fn standing_with_year(
    db: &State<ConnectionPool>,
    series: Series,
    param: DriverStandingParameter,
    year: Year,
) -> Result<Json<StandingsResponse>> {
    let (drivers_standings, pagination) =
        driver_standing_inner_handler(db, series, Some(year), param)?;

    let response = StandingsResponse {
        pagination,
        series,
        drivers_standings,
        constructors_standings: Vec::new(),
    };

    Ok(Json(response))
}

fn driver_standing_inner_handler(
    db: &State<ConnectionPool>,
    series: Series,
    year: Option<Year>,
    param: DriverStandingParameter,
) -> Result<(Vec<DriverStanding>, Pagination)> {
    let f = |conn: &mut PooledConnection<ConnectionManager<MysqlConnection>>| {
        let filter: DriverStandingFilter = match year {
            Some(year) => {
                let race = models::Race::last_race_of_year(year).first(conn)?;
                let mut filter: DriverStandingFilter = param.into();
                filter.race_id = Some(race.race_id.into());
                filter
            }
            None => param.into(),
        };

        let (standing_and_driver, pagination) =
            application::models::DriverStanding::filter(filter).load_and_count_pages(conn)?;

        Ok((
            standing_and_driver
                .into_iter()
                .map(models::Tuple::<models::DriverStanding, models::Driver>::from)
                .map(DriverStanding::from)
                .collect::<Vec<DriverStanding>>(),
            pagination,
        ))
    };

    let pool = &mut db.from_series(series).get()?;
    pool.transaction(f)
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![standing]
}
