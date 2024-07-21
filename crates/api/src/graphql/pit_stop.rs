use async_graphql::{Context, Object};

use application::SqlBuilder;
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

        let res = application::pit_stop::PitStopQueryBuilder::pit_stops(options)
            .query_pagination(pagination.unwrap_or_default(), conn)?;

        res.0.try_into()
    }
}
