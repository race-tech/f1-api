use axum::extract::{Json, Path, Query, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

pub async fn seasons(
    pool: State<ConnectionPool>,
    Path(series): Path<Series>,
    Query(params): Query<GetSeasonsParameters>,
) -> Result<Json<Response<VecResponse<Season>>>> {
    let conn = &mut pool.from_series(series).get()?;

    let response = match params.season {
        Some(season) => {
            let season = application::seasons::SeasonsQueryBuilder::get(season, conn)?;

            (season, series).into()
        }
        None => {
            let res =
                application::seasons::SeasonsQueryBuilder::params(params).query_and_count(conn)?;

            Response::from((res.0, series)).pagination(res.1)
        }
    };

    Ok(Json(response))
}
