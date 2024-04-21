use axum::extract::{Extension, Json, Path, Query};

use infrastructure::ConnectionPool;
use shared::prelude::*;

pub async fn constructors(
    Extension(pool): Extension<ConnectionPool>,
    Path(series): Path<Series>,
    Query(params): Query<GetConstructorsParameter>,
) -> Result<Json<Response<VecResponse<Constructor>>>> {
    let conn = &mut pool.from_series(series).get()?;

    let response = match params.constructor_ref {
        Some(constructor_ref) => {
            let constructor =
                application::constructors::ConstructorsQueryBuilder::get(constructor_ref, conn)?;

            (constructor, series).into()
        }
        None => {
            let res = application::constructors::ConstructorsQueryBuilder::params(params)
                .query_and_count(conn)?;

            Response::from((res.0, series)).pagination(res.1)
        }
    };

    Ok(Json(response))
}
