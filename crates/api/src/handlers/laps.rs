use axum::extract::{Extension, Json, Path, Query};

use infrastructure::ConnectionPool;
use shared::prelude::*;

pub async fn laps(
    Extension(pool): Extension<ConnectionPool>,
    Path(series): Path<Series>,
    Query(params): Query<GetLapsParameter>,
) -> Result<Json<Response<LapsResponse>>> {
    let conn = &mut pool.from_series(series).get()?;

    let res = application::laps::LapsQueryBuilder::params(params).query_and_count(conn)?;

    let response = Response {
        data: res.0.try_into()?,
        pagination: Some(res.1),
        series,
    };

    Ok(Json(response))
}
