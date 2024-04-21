use axum::{
    extract::{Json, Path, Query},
    Extension,
};

use infrastructure::ConnectionPool;
use shared::prelude::*;

pub async fn circuits(
    Extension(pool): Extension<ConnectionPool>,
    Path(series): Path<Series>,
    Query(params): Query<GetCircuitsParameter>,
) -> Result<Json<Response<VecResponse<Circuit>>>> {
    let conn = &mut pool.from_series(series).get()?;

    let response = match params.circuit_ref {
        Some(circuit_ref) => {
            let circuit = application::circuits::CircuitsQueryBuilder::get(circuit_ref, conn)?;

            (circuit, series).into()
        }
        None => {
            let res = application::circuits::CircuitsQueryBuilder::params(params)
                .query_and_count(conn)?;

            Response::from((res.0, series)).pagination(res.1)
        }
    };

    Ok(Json(response))
}
