use async_graphql::{Context, Object};

use shared::{
    error::Result,
    models::graphql::{Circuit, GetCircuitsOpts, PaginationOpts},
    models::response::Response,
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct CircuitQuery;

#[Object]
impl CircuitQuery {
    async fn circuit<'ctx>(&self, ctx: &Context<'ctx>, circuit_ref: String) -> Result<Circuit> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::circuit::CircuitQueryBuilder::get(circuit_ref, conn)?;
        Ok(res.into())
    }

    async fn circuits<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetCircuitsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Circuit>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::circuit::CircuitQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }
}
