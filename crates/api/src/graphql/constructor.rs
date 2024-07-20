use async_graphql::{Context, Object};

use application::SqlBuilder;
use shared::{
    error,
    error::Result,
    models::graphql::{Constructor, GetConstructorsOpts, PaginationOpts},
    models::response::Response,
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct ConstructorQuery;

#[Object]
impl ConstructorQuery {
    async fn constructor<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        constructor_ref: String,
    ) -> Result<Constructor> {
        let conn = &mut ctx.extract_conn()?;

        application::constructor::ConstructorQueryBuilder::constructor(&constructor_ref)
            .query_first(conn)?
            .map(Into::into)
            .ok_or(error!(EntityNotFound => "constructor not found"))
    }

    async fn constructors<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetConstructorsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Constructor>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::constructor::ConstructorQueryBuilder::constructors(
            options.unwrap_or_default(),
        )
        .query_pagination(pagination.unwrap_or_default(), conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }
}
