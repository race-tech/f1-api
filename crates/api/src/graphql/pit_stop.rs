use async_graphql::{Context, Object};

use shared::{
    error::Result,
    models::graphql::{GetPitStopsOpts, PaginationOpts, PitStops},
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct PitStopQuery;

#[Object]
impl PitStopQuery {
    async fn pit_stops<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: GetPitStopsOpts,
        pagination: Option<PaginationOpts>,
    ) -> Result<PitStops> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::pit_stop::PitStopQueryBuilder::params(
            (options, pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        res.0.try_into()
    }
}
