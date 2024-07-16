use async_graphql::{Context, Object};

use shared::{
    error::Result,
    models::graphql::{GetRacesOpts, PaginationOpts, Race},
    models::response::Response,
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct RaceQuery;

#[Object]
impl RaceQuery {
    async fn race<'ctx>(&self, ctx: &Context<'ctx>, year: u32, round: u32) -> Result<Option<Race>> {
        // SAFETY: This should always work
        let conn = &mut ctx.extract_conn()?;
        let params = shared::parameters::GetRacesParameters {
            year: Some(year),
            round: Some(round),
            limit: Some(1),
            page: Some(1),
            ..Default::default()
        };

        let res = application::race::RaceQueryBuilder::params(params).query_and_count(conn)?;

        Ok(res.0.into_iter().map(Into::into).next())
    }

    async fn races<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetRacesOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Race>>> {
        // SAFETY: This should always work
        let conn = &mut ctx.extract_conn()?;

        let res = application::race::RaceQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let races = res.0.into_iter().map(Into::into).collect();
        Ok((races, res.1).into())
    }
}
