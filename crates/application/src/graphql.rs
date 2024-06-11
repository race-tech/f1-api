use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

use infrastructure::ConnectionPool;
use shared::{
    models::graphql::{
        Circuit, Constructor, ConstructorStanding, Driver, DriverStanding, GetCircuitsOpts,
        GetConstructorStandingsOpts, GetConstructorsOpts, GetDriverStandingsOpts, GetDriversOpts,
        GetLapsOpts, GetPitStopsOpts, GetRacesOpts, GetSeasonsOpts, GetStatusOpts, Laps,
        Pagination, PitStops, Race, Season, Status, Wrapper,
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
            limit: Some(1),
            page: Some(1),
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
        options: Option<GetRacesOpts>,
        pagination: Option<Pagination>,
    ) -> Vec<Race> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::races::RacesQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        res.0.into_iter().map(Into::into).collect()
    }

    async fn circuit<'ctx>(&self, ctx: &Context<'ctx>, circuit_ref: String) -> Circuit {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::circuits::CircuitsQueryBuilder::get(circuit_ref, conn).unwrap();
        res.into()
    }

    async fn circuits<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetCircuitsOpts>,
        pagination: Option<Pagination>,
    ) -> Vec<Circuit> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::circuits::CircuitsQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        res.0.into_iter().map(Into::into).collect()
    }

    async fn constructors_standings<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetConstructorStandingsOpts>,
        pagination: Option<Pagination>,
    ) -> Vec<ConstructorStanding> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::constructor_standings::ConstructorStandingsQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
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
        options: Option<GetConstructorsOpts>,
        pagination: Option<Pagination>,
    ) -> Vec<Constructor> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::constructors::ConstructorsQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        res.0.into_iter().map(Into::into).collect()
    }

    async fn drivers_standings<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetDriverStandingsOpts>,
        pagination: Option<Pagination>,
    ) -> Vec<DriverStanding> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::driver_standings::DriverStandingsQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        let wrapper: Wrapper<DriverStanding> = res.0.into();
        wrapper.0
    }

    async fn driver<'ctx>(&self, ctx: &Context<'ctx>, driver_ref: String) -> Driver {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::drivers::DriversQueryBuilder::get(driver_ref, conn).unwrap();

        res.into()
    }

    async fn drivers<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetDriversOpts>,
        pagination: Option<Pagination>,
    ) -> Vec<Driver> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::drivers::DriversQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        res.0.into_iter().map(Into::into).collect()
    }

    async fn laps<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: GetLapsOpts,
        pagination: Option<Pagination>,
    ) -> Laps {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res =
            crate::laps::LapsQueryBuilder::params((options, pagination.unwrap_or_default()).into())
                .query_and_count(conn)
                .unwrap();

        res.0.try_into().unwrap()
    }

    async fn pit_stops<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: GetPitStopsOpts,
        pagination: Option<Pagination>,
    ) -> PitStops {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::pit_stops::PitStopsQueryBuilder::params(
            (options, pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        res.0.try_into().unwrap()
    }

    async fn season<'ctx>(&self, ctx: &Context<'ctx>, year: u32) -> Season {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::seasons::SeasonsQueryBuilder::get(year, conn).unwrap();

        res.into()
    }

    async fn seasons<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetSeasonsOpts>,
        pagination: Option<Pagination>,
    ) -> Vec<Season> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::seasons::SeasonsQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        res.0.into_iter().map(Into::into).collect()
    }

    async fn status<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetStatusOpts>,
        pagination: Option<Pagination>,
    ) -> Vec<Status> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.from_series(Series::F1).get().unwrap();

        let res = crate::status::StatusQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)
        .unwrap();

        res.0.into_iter().map(Into::into).collect()
    }
}
