use axum::extract::{Json, Path, Query, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

pub async fn races(
    pool: State<ConnectionPool>,
    Path(series): Path<Series>,
    Query(params): Query<GetRacesParameters>,
) -> Result<Json<Response<VecResponse<RaceResponse>>>> {
    let conn = &mut pool.from_series(series).get()?;

    let res = application::races::RacesQueryBuilder::params(params).query_and_count(conn)?;

    let response = Response::from((res.0, series)).pagination(res.1);

    Ok(Json(response))
}
