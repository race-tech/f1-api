use async_graphql::{Context, Object};

use application::SqlBuilder;
use shared::{
    error,
    error::Result,
    models::graphql::{Circuit, GetCircuitsOpts, PaginationOpts},
    models::response::Response,
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct CircuitQuery;

#[Object]
impl CircuitQuery {
    async fn circuit(&self, ctx: &Context<'_>, circuit_ref: String) -> Result<Circuit> {
        let conn = &mut ctx.extract_conn()?;

        application::circuit::CircuitQueryBuilder::circuit(&circuit_ref)
            .query_first(conn)?
            .map(Into::into)
            .ok_or(error!(EntityNotFound => "circuit not found"))
    }

    async fn circuits(
        &self,
        ctx: &Context<'_>,
        options: Option<GetCircuitsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Circuit>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::circuit::CircuitQueryBuilder::circuits(options.unwrap_or_default())
            .query_pagination(pagination.unwrap_or_default(), conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }
}
