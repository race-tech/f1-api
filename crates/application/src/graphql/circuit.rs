use async_graphql::{Context, Object};

use infrastructure::ConnectionPool;
use shared::{
    error,
    error::Result,
    models::graphql::{Circuit, GetCircuitsOpts, PaginationOpts},
    models::response::Response,
    models::surreal,
};

use crate::pagination::Paginate;

#[derive(Default)]
pub struct CircuitQuery;

#[Object]
impl CircuitQuery {
    async fn circuit<'ctx>(&self, ctx: &Context<'ctx>, circuit_ref: String) -> Result<Circuit> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get().await?;

        let query = crate::surreal::circuits::get(circuit_ref);
        let mut res = conn.query(query).await?;
        let circuit: Option<Circuit> = res.take::<Option<surreal::Circuit>>(0)?.map(Into::into);
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
            .paginate::<surreal::Circuit>(pagination.unwrap_or_default())
            .query_and_count(conn)
            .await?;
        let circuits = res.0.into_iter().map(Into::into).collect();

        Ok((circuits, res.1).into())
    }
}
