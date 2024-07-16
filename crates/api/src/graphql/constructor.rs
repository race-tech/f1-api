use async_graphql::{Context, Object};

use shared::{
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

        let res = application::constructor::ConstructorQueryBuilder::get(constructor_ref, conn)?;

        Ok(res.into())
    }

    async fn constructors<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetConstructorsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Constructor>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::constructor::ConstructorQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }
}
