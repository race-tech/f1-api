use async_graphql::{Context, Object};

use application::SqlBuilder;
use shared::{
    error,
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

        application::driver::DriverQueryBuilder::driver(&driver_ref)
            .query_first(conn)?
            .map(Into::into)
            .ok_or(error!(EntityNotFound => "driver not found"))
    }

    async fn drivers<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetDriversOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Driver>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::driver::DriverQueryBuilder::drivers(options.unwrap_or_default())
            .query_pagination(pagination.unwrap_or_default(), conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }
}
