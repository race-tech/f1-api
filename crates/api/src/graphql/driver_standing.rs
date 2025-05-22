use async_graphql::{Context, Object};

use application::SqlBuilder;
use shared::{
    error::Result,
    models::graphql::{DriverStanding, GetDriverStandingsOpts, PaginationOpts, Wrapper},
    models::response::Response,
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct DriverStandingQuery;

#[Object]
impl DriverStandingQuery {
    async fn drivers_standings(
        &self,
        ctx: &Context<'_>,
        options: Option<GetDriverStandingsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<DriverStanding>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::driver_standing::DriverStandingQueryBuilder::driver_standings(
            options.unwrap_or_default(),
        )
        .query_pagination(pagination.unwrap_or_default(), conn)?;

        let wrapper: Wrapper<DriverStanding> = res.0.into();
        Ok((wrapper.0, res.1).into())
    }
}
