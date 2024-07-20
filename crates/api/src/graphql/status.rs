use async_graphql::{Context, Object};

use application::SqlBuilder;
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

        let res = application::status::StatusQueryBuilder::statuses(options.unwrap_or_default())
            .query_pagination(pagination.unwrap_or_default(), conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }
}
