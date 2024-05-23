use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

use infrastructure::ConnectionPool;
use shared::{
    models::graphql::Race,
    parameters::{GetRacesParameters, Series},
};

pub type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;
pub struct Query;

#[Object]
impl Query {
    async fn races<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Race> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let params = GetRacesParameters::default();
        let res = crate::races::RacesQueryBuilder::params(params)
            .query_and_count(conn)
            .unwrap();

        res.0.into_iter().map(Into::into).collect()
    }
}
