use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

use infrastructure::ConnectionPool;
use shared::{
    error::Result,
    models::graphql::{
        Circuit, Constructor, ConstructorStanding, Driver, DriverStanding, GetCircuitsOpts,
        GetConstructorStandingsOpts, GetConstructorsOpts, GetDriverStandingsOpts, GetDriversOpts,
        GetLapsOpts, GetPitStopsOpts, GetRacesOpts, GetSeasonsOpts, GetStatusOpts, Laps,
        PaginationOpts, PitStops, Race, Season, Status, Wrapper,
    },
    models::response::Response,
};

pub type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;
pub struct Query;

#[Object]
impl Query {
    async fn race<'ctx>(&self, ctx: &Context<'ctx>, year: u32, round: u32) -> Result<Option<Race>> {
        // SAFETY: This should always work
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;
        let params = shared::parameters::GetRacesParameters {
            year: Some(year),
            round: Some(round),
            limit: Some(1),
            page: Some(1),
            ..Default::default()
        };

        let res = crate::races::RacesQueryBuilder::params(params).query_and_count(conn)?;

        Ok(res.0.into_iter().map(Into::into).next())
    }

    async fn races<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetRacesOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Race>>> {
        // SAFETY: This should always work
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::races::RacesQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let races = res.0.into_iter().map(Into::into).collect();
        Ok((races, res.1).into())
    }

    async fn circuit<'ctx>(&self, ctx: &Context<'ctx>, circuit_ref: String) -> Result<Circuit> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get().await?;

        let res = crate::circuits::CircuitsQueryBuilder::get(circuit_ref);
        let res = conn.query(res.0).bind(res.1).await?;
        Ok(res.into())
    }

    async fn circuits<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetCircuitsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Circuit>>> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::circuits::CircuitsQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }

    async fn constructors_standings<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetConstructorStandingsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<ConstructorStanding>>> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::constructor_standings::ConstructorStandingsQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let wrapper: Wrapper<ConstructorStanding> = res.0.into();
        Ok((wrapper.0, res.1).into())
    }

    async fn constructor<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        constructor_ref: String,
    ) -> Result<Constructor> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::constructors::ConstructorsQueryBuilder::get(constructor_ref, conn)?;

        Ok(res.into())
    }

    async fn constructors<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetConstructorsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Constructor>>> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::constructors::ConstructorsQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }

    async fn drivers_standings<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetDriverStandingsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<DriverStanding>>> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::driver_standings::DriverStandingsQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let wrapper: Wrapper<DriverStanding> = res.0.into();
        Ok((wrapper.0, res.1).into())
    }

    async fn driver<'ctx>(&self, ctx: &Context<'ctx>, driver_ref: String) -> Result<Driver> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::drivers::DriversQueryBuilder::get(driver_ref, conn)?;

        Ok(res.into())
    }

    async fn drivers<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetDriversOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Driver>>> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::drivers::DriversQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }

    async fn laps<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: GetLapsOpts,
        pagination: Option<PaginationOpts>,
    ) -> Result<Laps> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res =
            crate::laps::LapsQueryBuilder::params((options, pagination.unwrap_or_default()).into())
                .query_and_count(conn)?;

        res.0.try_into()
    }

    async fn pit_stops<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: GetPitStopsOpts,
        pagination: Option<PaginationOpts>,
    ) -> Result<PitStops> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::pit_stops::PitStopsQueryBuilder::params(
            (options, pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        res.0.try_into()
    }

    async fn season<'ctx>(&self, ctx: &Context<'ctx>, year: u32) -> Result<Season> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::seasons::SeasonsQueryBuilder::get(year, conn)?;

        Ok(res.into())
    }

    async fn seasons<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetSeasonsOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Season>>> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::seasons::SeasonsQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }

    async fn status<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        options: Option<GetStatusOpts>,
        pagination: Option<PaginationOpts>,
    ) -> Result<Response<Vec<Status>>> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let conn = &mut pool.pool.get()?;

        let res = crate::status::StatusQueryBuilder::params(
            (options.unwrap_or_default(), pagination.unwrap_or_default()).into(),
        )
        .query_and_count(conn)?;

        let data = res.0.into_iter().map(Into::into).collect();
        Ok((data, res.1).into())
    }
}
