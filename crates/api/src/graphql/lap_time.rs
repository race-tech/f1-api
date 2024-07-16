use async_graphql::{Context, Object};

use shared::{
    error::Result,
    models::graphql::{GetLapsOpts, Laps, PaginationOpts},
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct LapTimeQuery;

#[Object]
impl LapTimeQuery {
    async fn lap_times<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: GetLapsOpts,
        pagination: Option<PaginationOpts>,
    ) -> Result<Laps> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::lap_time::LapTimeQueryBuilder::params(
            (options, pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        res.0.try_into()
    }
}
