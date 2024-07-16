use async_graphql::{Context, Object};

use shared::{
    error::Result,
    models::graphql::{Driver, GetDriversOpts, PaginationOpts},
    models::response::Response,
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct DriverQuery;

#[Object]
impl DriverQuery {
    async fn driver<'ctx>(&self, ctx: &Context<'ctx>, driver_ref: String) -> Result<Driver> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::driver::DriverQueryBuilder::get(driver_ref, conn)?;

        Ok(res.into())
    }

    async fn drivers<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetDriversOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Driver>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::driver::DriverQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }
}
