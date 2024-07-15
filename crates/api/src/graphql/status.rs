use async_graphql::{Context, Object};

use shared::{
    error::Result,
    models::graphql::{GetStatusOpts, PaginationOpts, Status},
    models::response::Response,
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct StatusQuery;

#[Object]
impl StatusQuery {
    async fn status<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetStatusOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Status>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::status::StatusQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }
}
