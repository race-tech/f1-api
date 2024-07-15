use async_graphql::{Context, Object};

use infrastructure::ConnectionPool;
use shared::{
    error::Result,
    models::graphql::{ConstructorStanding, GetConstructorStandingsOpts, PaginationOpts, Wrapper},
    models::response::Response,
    models::surreal,
};

use crate::pagination::Paginate;

#[derive(Default)]
pub struct ConstructorStandingQuery;

#[Object]
impl ConstructorStandingQuery {
    async fn constructors_standings<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetConstructorStandingsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<ConstructorStanding>>> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get().await?;

        let res =
            crate::surreal::constructor_standings::get_with_params(options.unwrap_or_default())
                .paginate::<surreal::ConstructorStanding>(pagination.unwrap_or_default())
                .query_and_count(conn)
                .await?;

        let standings: Wrapper<ConstructorStanding> = res.0.into();
        Ok((standings.0, res.1).into())
    }
}
