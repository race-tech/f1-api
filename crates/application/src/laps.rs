use sea_query::{Alias, Expr, Func, Query, SelectStatement};

use shared::models::Lap as LapModel;
use shared::parameters::GetLapsParameter;

use crate::{
    iden::*,
    pagination::{Paginate, Paginated},
    sql::SqlBuilder,
};

pub struct LapsQueryBuilder {
    stmt: SelectStatement,
    params: GetLapsParameter,
}

impl LapsQueryBuilder {
    pub fn params(params: GetLapsParameter) -> Self {
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
            .column((LapTimes::Table, LapTimes::Lap))
            .column((LapTimes::Table, LapTimes::Position))
            .column((LapTimes::Table, LapTimes::Time))
            .from(LapTimes::Table)
            .from(Races::Table)
            .from(Circuits::Table)
            .from(Drivers::Table)
            .and_where(
                Expr::col((Races::Table, Races::CircuitId))
                    .equals((Circuits::Table, Circuits::CircuitId)),
            )
            .and_where(
                Expr::col((LapTimes::Table, LapTimes::DriverId))
                    .equals((Drivers::Table, Drivers::DriverId)),
            )
            .and_where(
                Expr::col((LapTimes::Table, LapTimes::RaceId))
                    .equals((Races::Table, Races::RaceId)),
            )
            .and_where(Expr::col((Races::Table, Races::Year)).eq(Expr::value(*params.year)))
            .and_where(Expr::col((Races::Table, Races::Round)).eq(Expr::value(*params.round)))
            .order_by((LapTimes::Table, LapTimes::Lap), sea_query::Order::Asc)
            .order_by((LapTimes::Table, LapTimes::Position), sea_query::Order::Asc)
            .to_owned();

        Self { stmt, params }
    }

    pub fn build(self) -> Paginated<LapModel> {
        let page: u64 = self.params.page.unwrap_or_default().0;
        let limit: u64 = self.params.limit.unwrap_or_default().0;

        self.and_where(|s| {
            s.params
                .driver_ref
                .as_ref()
                .map(|d| Expr::col((Drivers::Table, Drivers::DriverRef)).eq(Expr::value(&**d)))
        })
        .and_where(|s| {
            s.params
                .lap_number
                .map(|n| Expr::col((LapTimes::Table, LapTimes::Lap)).eq(Expr::value(*n)))
        })
        .stmt
        .paginate(page)
        .per_page(limit)
    }
}

impl SqlBuilder for LapsQueryBuilder {
    fn stmt(&mut self) -> &mut sea_query::SelectStatement {
        &mut self.stmt
    }
}
