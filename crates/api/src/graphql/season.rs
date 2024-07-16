use async_graphql::{Context, Object};

use shared::{
    error::Result,
    models::graphql::{GetSeasonsOpts, PaginationOpts, Season},
    models::response::Response,
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct SeasonQuery;

#[Object]
impl SeasonQuery {
    async fn season<'ctx>(&self, ctx: &Context<'ctx>, year: u32) -> Result<Season> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::season::SeasonQueryBuilder::get(year, conn)?;

        Ok(res.into())
    }

    async fn seasons<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetSeasonsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Season>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::season::SeasonQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }
}
