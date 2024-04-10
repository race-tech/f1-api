use diesel::{Connection, RunQueryDsl};
use rocket::serde::json::Json;
use rocket::{get, routes, State};

use application;
use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/circuits?<param..>")]
pub fn circuits(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetCircuitsParameter,
) -> Result<()> {
    let pool = &mut db.from_series(series).get().unwrap();

    let query = application::circuit::CircuitQueryBuilder::filter(param.into())
        .build()
        .unwrap();

    let res = pool.transaction(|conn| query.load::<shared::models::(conn)).unwrap();

    Ok(())
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![circuits]
}
