use sea_query::{Alias, Expr, Func, Query, SelectStatement};

use shared::error::Result;
use shared::{models::Race as RaceModel, parameters::GetRacesParameters};

use crate::{
    iden::*,
    one_of,
    pagination::{Paginate, Paginated},
    sql::SqlBuilder,
};

pub struct RaceQueryBuilder {
    stmt: SelectStatement,
    params: GetRacesParameters,
}

const DATE_AND_TIME_COLS: &[(Races, &str)] = &[
    (Races::Date, "date"),
    (Races::Time, "time"),
    (Races::Fp1Date, "fp1_date"),
    (Races::Fp1Time, "fp1_time"),
    (Races::Fp2Date, "fp2_date"),
    (Races::Fp2Time, "fp2_time"),
    (Races::Fp3Date, "fp3_date"),
    (Races::Fp3Time, "fp3_time"),
    (Races::QualiDate, "quali_date"),
    (Races::QualiTime, "quali_time"),
    (Races::SprintDate, "sprint_date"),
    (Races::SprintTime, "sprint_time"),
];

impl RaceQueryBuilder {
    pub fn race(year: u32, round: u32) -> Self {
        let stmt = Query::select()
            .distinct()
            .column((Races::Table, Races::Year))
            .column((Races::Table, Races::Round))
            .expr_as(
                Expr::col((Races::Table, Races::Name)),
                Alias::new("raceName"),
            )
            .expr_as(Expr::col((Races::Table, Races::Url)), Alias::new("raceUrl"))
            .to_owned();

        let stmt = DATE_AND_TIME_COLS
            .windows(2)
            .step_by(2)
            .fold(stmt, |mut stmt, w| {
                let (col1, alias1) = w[0];
                let (col2, alias2) = w[1];
                stmt.expr_as(
                    Func::cust(Alias::new("DATE_FORMAT"))
                        .arg(Expr::col((Races::Table, col1)))
                        .arg("%Y-%m-%d"),
                    Alias::new(alias1),
                )
                .expr_as(
                    Func::cust(Alias::new("DATE_FORMAT"))
                        .arg(Expr::col((Races::Table, col2)))
                        .arg("%H:%i:%S"),
                    Alias::new(alias2),
                )
                .to_owned()
            })
            .column((Circuits::Table, Circuits::CircuitRef))
            .column((Circuits::Table, Circuits::Name))
            .column((Circuits::Table, Circuits::Location))
            .column((Circuits::Table, Circuits::Country))
            .column((Circuits::Table, Circuits::Lat))
            .column((Circuits::Table, Circuits::Lng))
            .column((Circuits::Table, Circuits::Alt))
            .column((Circuits::Table, Circuits::Url))
            .from(Races::Table)
            .from(Circuits::Table)
            .and_where(
                Expr::col((Races::Table, Races::CircuitId))
                    .equals((Circuits::Table, Circuits::CircuitId)),
            )
            .and_where(Expr::col((Races::Table, Races::Year)).eq(year))
            .and_where(Expr::col((Races::Table, Races::Round)).eq(round))
            .order_by((Races::Table, Races::Year), sea_query::Order::Asc)
            .order_by((Races::Table, Races::Round), sea_query::Order::Asc)
            .limit(1)
            .to_owned();

        Self {
            stmt,
            params: GetRacesParameters::default(),
        }
    }

    pub fn latest_race() -> Result<Self> {
        let now = time::OffsetDateTime::now_utc();
        let date = now.format(shared::DATE_FORMAT)?;

        let stmt = Query::select()
            .distinct()
            .column((Races::Table, Races::Year))
            .column((Races::Table, Races::Round))
            .expr_as(
                Expr::col((Races::Table, Races::Name)),
                Alias::new("raceName"),
            )
            .expr_as(Expr::col((Races::Table, Races::Url)), Alias::new("raceUrl"))
            .to_owned();

        let stmt = DATE_AND_TIME_COLS
            .windows(2)
            .step_by(2)
            .fold(stmt, |mut stmt, w| {
                let (col1, alias1) = w[0];
                let (col2, alias2) = w[1];
                stmt.expr_as(
                    Func::cust(Alias::new("DATE_FORMAT"))
                        .arg(Expr::col((Races::Table, col1)))
                        .arg("%Y-%m-%d"),
                    Alias::new(alias1),
                )
                .expr_as(
                    Func::cust(Alias::new("DATE_FORMAT"))
                        .arg(Expr::col((Races::Table, col2)))
                        .arg("%H:%i:%S"),
                    Alias::new(alias2),
                )
                .to_owned()
            })
            .column((Circuits::Table, Circuits::CircuitRef))
            .column((Circuits::Table, Circuits::Name))
            .column((Circuits::Table, Circuits::Location))
            .column((Circuits::Table, Circuits::Country))
            .column((Circuits::Table, Circuits::Lat))
            .column((Circuits::Table, Circuits::Lng))
            .column((Circuits::Table, Circuits::Alt))
            .column((Circuits::Table, Circuits::Url))
            .from(Races::Table)
            .from(Circuits::Table)
            .and_where(
                Expr::col((Races::Table, Races::CircuitId))
                    .equals((Circuits::Table, Circuits::CircuitId)),
            )
            .and_where(Expr::col((Races::Table, Races::Date)).lt(date))
            .order_by((Races::Table, Races::Date), sea_query::Order::Desc)
            .limit(1)
            .to_owned();

        Ok(Self {
            stmt,
            params: GetRacesParameters::default(),
        })
    }

    pub fn params(params: GetRacesParameters) -> Paginated<RaceModel> {
        let stmt = Query::select()
            .distinct()
            .column((Races::Table, Races::Year))
            .column((Races::Table, Races::Round))
            .expr_as(
                Expr::col((Races::Table, Races::Name)),
                Alias::new("raceName"),
            )
            .expr_as(Expr::col((Races::Table, Races::Url)), Alias::new("raceUrl"))
            .to_owned();

        let stmt = DATE_AND_TIME_COLS
            .windows(2)
            .step_by(2)
            .fold(stmt, |mut stmt, w| {
                let (col1, alias1) = w[0];
                let (col2, alias2) = w[1];
                stmt.expr_as(
                    Func::cust(Alias::new("DATE_FORMAT"))
                        .arg(Expr::col((Races::Table, col1)))
                        .arg("%Y-%m-%d"),
                    Alias::new(alias1),
                )
                .expr_as(
                    Func::cust(Alias::new("DATE_FORMAT"))
                        .arg(Expr::col((Races::Table, col2)))
                        .arg("%H:%i:%S"),
                    Alias::new(alias2),
                )
                .to_owned()
            })
            .column((Circuits::Table, Circuits::CircuitRef))
            .column((Circuits::Table, Circuits::Name))
            .column((Circuits::Table, Circuits::Location))
            .column((Circuits::Table, Circuits::Country))
            .column((Circuits::Table, Circuits::Lat))
            .column((Circuits::Table, Circuits::Lng))
            .column((Circuits::Table, Circuits::Alt))
            .column((Circuits::Table, Circuits::Url))
            .from(Races::Table)
            .from(Circuits::Table)
            .and_where(
                Expr::col((Races::Table, Races::CircuitId))
                    .equals((Circuits::Table, Circuits::CircuitId)),
            )
            .order_by((Races::Table, Races::Year), sea_query::Order::Asc)
            .order_by((Races::Table, Races::Round), sea_query::Order::Asc)
            .to_owned();

        let stmt = stmt.to_owned();

        Self { stmt, params }.build()
    }

    fn build(self) -> Paginated<RaceModel> {
        let page: u64 = self.params.page.unwrap_or_default();
        let limit: u64 = self.params.limit.unwrap_or_default();

        self.from(
            |s| {
                one_of!(
                    s.params.driver_ref,
                    s.params.constructor_ref,
                    s.params.grid,
                    s.params.result,
                    s.params.status,
                    s.params.fastest
                )
            },
            Results::Table,
        )
        .from(|s| s.params.driver_ref.is_some(), Drivers::Table)
        .from(|s| s.params.constructor_ref.is_some(), Constructors::Table)
        .and_where(|s| {
            s.params
                .year
                .map(|y| Expr::col((Races::Table, Races::Year)).eq(Expr::val(y)))
        })
        .and_where(|s| {
            s.params
                .round
                .map(|r| Expr::col((Races::Table, Races::Round)).eq(Expr::val(r)))
        })
        .and_where(|s| {
            s.params
                .circuit_ref
                .as_ref()
                .map(|c| Expr::col((Circuits::Table, Circuits::CircuitRef)).eq(Expr::val(&**c)))
        })
        .and_where(|s| {
            one_of!(
                s.params.driver_ref,
                s.params.constructor_ref,
                s.params.grid,
                s.params.result,
                s.params.status,
                s.params.fastest
            )
            .then_some(
                Expr::col((Races::Table, Races::RaceId)).equals((Results::Table, Results::RaceId)),
            )
        })
        .and_where(|s| {
            s.params.constructor_ref.as_ref().map(|c| {
                Expr::col((Constructors::Table, Constructors::ConstructorRef))
                    .eq(Expr::val(&**c))
                    .and(
                        Expr::col((Constructors::Table, Constructors::ConstructorId))
                            .equals((Results::Table, Results::ConstructorId)),
                    )
            })
        })
        .and_where(|s| {
            s.params.driver_ref.as_ref().map(|c| {
                Expr::col((Drivers::Table, Drivers::DriverRef))
                    .eq(Expr::val(&**c))
                    .and(
                        Expr::col((Drivers::Table, Drivers::DriverId))
                            .equals((Results::Table, Results::DriverId)),
                    )
            })
        })
        .and_where(|s| {
            s.params
                .status
                .map(|s| Expr::col((Results::Table, Results::StatusId)).eq(Expr::value(s)))
        })
        .and_where(|s| {
            s.params
                .grid
                .map(|g| Expr::col((Results::Table, Results::Grid)).eq(Expr::value(g)))
        })
        .and_where(|s| {
            s.params
                .fastest
                .map(|f| Expr::col((Results::Table, Results::Rank)).eq(Expr::value(f)))
        })
        .and_where(|s| {
            s.params
                .result
                .map(|r| Expr::col((Results::Table, Results::PositionText)).eq(Expr::value(r)))
        })
        .stmt
        .paginate(page)
        .per_page(limit)
    }
}

impl SqlBuilder for RaceQueryBuilder {
    type Output = RaceModel;

    fn stmt(&mut self) -> &mut sea_query::SelectStatement {
        &mut self.stmt
    }
}
