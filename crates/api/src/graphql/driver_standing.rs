use async_graphql::{Context, Object};

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
    async fn drivers_standings<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetDriverStandingsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<DriverStanding>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::driver_standing::DriverStandingQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let wrapper: Wrapper<DriverStanding> = res.0.into();
        Ok((wrapper.0, res.1).into())
    }
}
