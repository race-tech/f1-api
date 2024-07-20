use sea_query::{Alias, Expr, Func, Query, SelectStatement};

use shared::models::graphql::GetPitStopsOpts;
use shared::models::PitStop as PitStopModel;

use crate::{iden::*, sql::SqlBuilder};

pub struct PitStopQueryBuilder<P> {
    stmt: SelectStatement,
    params: P,
}

impl PitStopQueryBuilder<GetPitStopsOpts> {
    pub fn pit_stops(params: GetPitStopsOpts) -> Self {
        let stmt = Query::select()
            .distinct()
            .expr_as(
                Expr::col((Races::Table, Races::Name)),
                Alias::new("raceName"),
            )
            .expr_as(
                Func::cust(Alias::new("DATE_FORMAT"))
                    .arg(Expr::col((Races::Table, Races::Date)))
                    .arg("%Y-%m-%d"),
                Alias::new("raceDate"),
            )
            .expr_as(
                Func::cust(Alias::new("DATE_FORMAT"))
                    .arg(Expr::col((Races::Table, Races::Time)))
                    .arg("%H:%i:%S"),
                Alias::new("raceTime"),
            )
            .expr_as(Expr::col((Races::Table, Races::Url)), Alias::new("raceUrl"))
            .column((Circuits::Table, Circuits::CircuitRef))
            .expr_as(
                Expr::col((Circuits::Table, Circuits::Name)),
                Alias::new("circuitName"),
            )
            .expr_as(
                Expr::col((Circuits::Table, Circuits::Location)),
                Alias::new("circuitLocation"),
            )
            .expr_as(
                Expr::col((Circuits::Table, Circuits::Country)),
                Alias::new("circuitCountry"),
            )
            .expr_as(
                Expr::col((Circuits::Table, Circuits::Lat)),
                Alias::new("circuitLat"),
            )
            .expr_as(
                Expr::col((Circuits::Table, Circuits::Lng)),
                Alias::new("circuitLng"),
            )
            .expr_as(
                Expr::col((Circuits::Table, Circuits::Alt)),
                Alias::new("circuitAlt"),
            )
            .expr_as(
                Expr::col((Circuits::Table, Circuits::Url)),
                Alias::new("circuitUrl"),
            )
            .column((Drivers::Table, Drivers::DriverRef))
            .column((PitStops::Table, PitStops::Stop))
            .column((PitStops::Table, PitStops::Lap))
            .expr_as(
                Func::cust(Alias::new("DATE_FORMAT"))
                    .arg(Expr::col((PitStops::Table, PitStops::Time)))
                    .arg("%H:%i:%S"),
                Alias::new("time"),
            )
            .column((PitStops::Table, PitStops::Duration))
            .from(PitStops::Table)
            .from(Races::Table)
            .from(Circuits::Table)
            .from(Drivers::Table)
            .and_where(
                Expr::col((Races::Table, Races::CircuitId))
                    .equals((Circuits::Table, Circuits::CircuitId)),
            )
            .and_where(
                Expr::col((PitStops::Table, PitStops::DriverId))
                    .equals((Drivers::Table, Drivers::DriverId)),
            )
            .and_where(
                Expr::col((PitStops::Table, PitStops::RaceId))
                    .equals((Races::Table, Races::RaceId)),
            )
            .and_where(Expr::col((Races::Table, Races::Year)).eq(Expr::val(params.year)))
            .and_where(Expr::col((Races::Table, Races::Round)).eq(Expr::val(params.round)))
            .order_by(PitStops::Time, sea_query::Order::Asc)
            .to_owned();

        Self { stmt, params }.build()
    }

    fn build(self) -> Self {
        self.and_where(|s| {
            s.params
                .driver_ref
                .as_ref()
                .map(|d| Expr::col((Drivers::Table, Drivers::DriverRef)).eq(Expr::value(d)))
        })
        .and_where(|s| {
            s.params
                .lap_number
                .map(|n| Expr::col((PitStops::Table, PitStops::Lap)).eq(Expr::value(n)))
        })
        .and_where(|s| {
            s.params
                .pit_stop_number
                .map(|n| Expr::col((PitStops::Table, PitStops::Stop)).eq(Expr::value(n)))
        })
    }
}

impl<P> SqlBuilder for PitStopQueryBuilder<P> {
    type Output = PitStopModel;

    fn stmt(&mut self) -> &mut sea_query::SelectStatement {
        &mut self.stmt
    }
}
