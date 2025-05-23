use async_graphql::{Context, Object};

use application::SqlBuilder;
use shared::{
    error,
    error::Result,
    models::graphql::{GetSeasonsOpts, PaginationOpts, Season},
    models::response::Response,
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct SeasonQuery;

#[Object]
impl SeasonQuery {
    async fn season(&self, ctx: &Context<'_>, year: u32) -> Result<Season> {
        let conn = &mut ctx.extract_conn()?;

        application::season::SeasonQueryBuilder::season(year)
            .query_first(conn)?
            .map(Into::into)
            .ok_or(error!(EntityNotFound => "season not found"))
    }

    async fn seasons(
        &self,
        ctx: &Context<'_>,
        options: Option<GetSeasonsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Season>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::season::SeasonQueryBuilder::seasons(options.unwrap_or_default())
            .query_pagination(pagination.unwrap_or_default(), conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }
}
