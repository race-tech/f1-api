use async_graphql::{Context, Object};

use application::SqlBuilder;
use shared::{
    error,
    error::Result,
    models::graphql::{GetRacesOpts, PaginationOpts, Race},
    models::response::Response,
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct RaceQuery;

#[Object]
impl RaceQuery {
    async fn latest_race<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Race> {
        // SAFETY: This should always work
        let conn = &mut ctx.extract_conn()?;

        application::race::RaceQueryBuilder::latest_race()?
            .query_first(conn)?
            .map(Into::into)
            .ok_or(error!(InternalServer => "latest race not found"))
    }

    async fn race<'ctx>(&self, ctx: &Context<'ctx>, year: u32, round: u32) -> Result<Race> {
        // SAFETY: This should always work
        let conn = &mut ctx.extract_conn()?;

        application::race::RaceQueryBuilder::race(year, round)
            .query_first(conn)?
            .map(Into::into)
            .ok_or(error!(EntityNotFound => "race not found"))
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
