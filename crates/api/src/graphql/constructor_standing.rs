use async_graphql::{Context, Object};

use shared::{
    error::Result,
    models::graphql::{ConstructorStanding, GetConstructorStandingsOpts, PaginationOpts, Wrapper},
    models::response::Response,
};

use crate::extension::ExtractConn;

#[derive(Default)]
pub struct ConstructorStandingQuery;

#[Object]
impl ConstructorStandingQuery {
    async fn constructors_standings<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetConstructorStandingsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<ConstructorStanding>>> {
        let conn = &mut ctx.extract_conn()?;

        let res = application::constructor_standing::ConstructorStandingQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let wrapper: Wrapper<ConstructorStanding> = res.0.into();
        Ok((wrapper.0, res.1).into())
    }
}
