use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

use infrastructure::ConnectionPool;
use shared::{
    error,
    error::Result,
    models::graphql::{Circuit, GetCircuitsOpts, PaginationOpts},
    models::response::Response,
};

use crate::pagination::Paginate;

pub type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;
#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn circuit<'ctx>(&self, ctx: &Context<'ctx>, circuit_ref: String) -> Result<Circuit> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get().await?;

        let res = crate::surreal::circuits::get(circuit_ref);
        let mut res = conn.query(res).await?;
        let circuit: Option<Circuit> = res.take(0)?;
        circuit.ok_or_else(|| error!(EntityNotFound => "Circuit not found"))
    }

    // TODO: check the options before querying the database
    async fn circuits<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetCircuitsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Circuit>>> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get().await?;

        let res = crate::surreal::circuits::get_with_params(options.unwrap_or_default())
            .paginate(pagination.unwrap_or_default())
            .query_and_count(conn)
            .await?;

        Ok((res.0, res.1).into())
    }
}
