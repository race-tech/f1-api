use async_graphql::{Context, Object};

use application::SqlBuilder;
use shared::{
    error::Result,
    models::graphql::{GetLapsOpts, Laps, PaginationOpts},
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct LapTimeQuery;

#[Object]
impl LapTimeQuery {
    async fn lap_times(
        &self,
        ctx: &Context<'_>,
        options: GetLapsOpts,
        pagination: Option<PaginationOpts>,
    ) -> Result<Laps> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::lap_time::LapTimeQueryBuilder::lap_times(options)
            .query_pagination(pagination.unwrap_or_default(), conn)?;

        res.0.try_into()
    }
}
