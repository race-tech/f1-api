use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

use infrastructure::ConnectionPool;
use shared::{
    error::Result,
    models::graphql::{ConstructorStanding, GetConstructorStandingsOpts, PaginationOpts, Wrapper},
    models::response::Response,
};

use crate::pagination::Paginate;

pub type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;
#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
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
                .paginate(pagination.unwrap_or_default())
                .query_and_count(conn)
                .await?;

        let wrapper: Wrapper<ConstructorStanding> = res.0.into();
        Ok((wrapper.0, res.1).into())
    }
}
