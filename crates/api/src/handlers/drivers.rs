use axum::extract::{Json, Path, Query, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

pub async fn drivers(
    pool: State<ConnectionPool>,
    Path(series): Path<Series>,
    Query(params): Query<GetDriversParameter>,
) -> Result<Json<Response<VecResponse<Driver>>>> {
    let conn = &mut pool.from_series(series).get()?;

    let response = match params.driver_ref {
        Some(driver_ref) => {
            let driver = application::drivers::DriversQueryBuilder::get(driver_ref, conn)?;

            (driver, series).into()
        }
        None => {
            let res =
                application::drivers::DriversQueryBuilder::params(params).query_and_count(conn)?;

            Response::from((res.0, series)).pagination(res.1)
        }
    };

    Ok(Json(response))
}
