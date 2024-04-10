use diesel::{Connection, RunQueryDsl};
use mysql::prelude::Queryable;
use mysql::Pool;
use rocket::serde::json::Json;
use rocket::{get, routes, State};

use application;
use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/circuits?<param..>")]
pub fn circuits(
    db: &State<Pool>,
    series: Series,
    param: shared::parameters::GetCircuitsParameter,
) -> Result<()> {
    let mut conn = db.get_conn().unwrap();

    let query = application::circuit::CircuitQueryBuilder::filter(param.into()).build();

    let res: Vec<application::models::Circuits> = conn.query(query).unwrap();

    println!("{:?}", res);

    Ok(())
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![circuits]
}
