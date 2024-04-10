use diesel::{Connection, RunQueryDsl};
use rocket::{get, routes, State};

use application;
use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/circuits?<param..>")]
pub fn circuits(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetCircuitsQueryParams,
) -> Result<()> {
    let pool = &mut db.from_series(series).get()?;
    let res = pool.transaction(|conn| {
        let query = application::circuit::get_circuits(param.into());
        diesel::sql_query(query).execute(conn)
    });

    println!("{:?}", res);

    Ok(())
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![circuits]
}
