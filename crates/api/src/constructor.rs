use rocket::serde::json::Json;
use rocket::{get, routes, State};

use application::{self, models::Constructors};
use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/constructors?<param..>")]
pub fn constructors(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetConstructorsParameter,
) -> Result<Json<Response<Vec<Constructors>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let query = application::constructor::ConstructorQueryBuilder::filter(param.into()).build();

    let res = query.query_and_count::<Constructors>(conn);

    let response = Response {
        data: res.0,
        pagination: res.1,
        series,
    };

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![constructors]
}
