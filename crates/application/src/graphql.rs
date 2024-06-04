use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

use infrastructure::ConnectionPool;
use shared::{
    models::graphql::{
        Circuit, Constructor, ConstructorStanding, DriverStanding, GetCircuitsOpts,
        GetConstructorStandingsOpts, GetConstructorsOpts, GetDriverStandingsOpts, GetRacesOpts,
        Pagination, Race, Wrapper,
    },
    parameters::Series,
};

pub type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;
pub struct Query;

#[Object]
impl Query {
    async fn race<'ctx>(&self, ctx: &Context<'ctx>, year: u32, round: u32) -> Option<Race> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();
        let params = shared::parameters::GetRacesParameters {
            year: Some(year),
            round: Some(round),
            ..Default::default()
        };

        let res = crate::races::RacesQueryBuilder::params(params)
            .query_and_count(conn)
            .unwrap();

        res.0.into_iter().map(Into::into).next()
    }

    async fn races<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: GetRacesOpts,
        pagination: Option<Pagination>,
    ) -> Vec<Race> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::races::RacesQueryBuilder::params(
            (options, pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        res.0.into_iter().map(Into::into).collect()
    }

    async fn circuit<'ctx>(&self, ctx: &Context<'ctx>, circuit_ref: String) -> Option<Circuit> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();
        let params = shared::parameters::GetCircuitsParameters {
            circuit_ref: Some(circuit_ref),
            ..Default::default()
        };

        let res = crate::circuits::CircuitsQueryBuilder::params(params)
            .query_and_count(conn)
            .unwrap();

        res.0.into_iter().map(Into::into).next()
    }

    async fn circuits<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: GetCircuitsOpts,
        pagination: Option<Pagination>,
    ) -> Vec<Circuit> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::circuits::CircuitsQueryBuilder::params(
            (options, pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        res.0.into_iter().map(Into::into).collect()
    }

    async fn constructors_standings<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: GetConstructorStandingsOpts,
        pagination: Option<Pagination>,
    ) -> Vec<ConstructorStanding> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::constructor_standings::ConstructorStandingsQueryBuilder::params(
            (options, pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        let wrapper: Wrapper<ConstructorStanding> = res.0.into();
        wrapper.0
    }

    async fn constructor<'ctx>(&self, ctx: &Context<'ctx>, constructor_ref: String) -> Constructor {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res =
            crate::constructors::ConstructorsQueryBuilder::get(constructor_ref, conn).unwrap();

        res.into()
    }

    async fn constructors<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: GetConstructorsOpts,
        pagination: Option<Pagination>,
    ) -> Vec<Constructor> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::constructors::ConstructorsQueryBuilder::params(
            (options, pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        res.0.into_iter().map(Into::into).collect()
    }

    async fn drivers_standings<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: GetDriverStandingsOpts,
        pagination: Option<Pagination>,
    ) -> Vec<DriverStanding> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::driver_standings::DriverStandingsQueryBuilder::params(
            (options, pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        let wrapper: Wrapper<DriverStanding> = res.0.into();
        wrapper.0
    }
}
