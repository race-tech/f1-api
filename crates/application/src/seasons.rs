use mysql::prelude::Queryable;
use sea_query::{Expr, Func, MysqlQueryBuilder, Query, SelectStatement};

use shared::error;
use shared::error::Result;
use shared::models::Season as SeasonModel;
use shared::parameters::GetSeasonsParameters;

use crate::{
    iden::*,
    one_of,
    pagination::{Paginate, Paginated},
    sql::SqlBuilder,
};

pub struct SeasonsQueryBuilder {
    stmt: SelectStatement,
    params: GetSeasonsParameters,
}

impl SeasonsQueryBuilder {
    pub fn get(season: u32, conn: &mut infrastructure::Connection) -> Result<SeasonModel> {
        let query = Query::select()
            .distinct()
            .column((Seasons::Table, Seasons::Year))
            .column((Seasons::Table, Seasons::Url))
            .from(Seasons::Table)
            .and_where(Expr::col((Seasons::Table, Seasons::Year)).eq(Expr::value(season)))
            .to_string(MysqlQueryBuilder);

        conn.query_first(query)?
            .ok_or(error!(EntityNotFound => "season with year `{}` not found", season))
    }

    pub fn params(params: GetSeasonsParameters) -> Paginated<SeasonModel> {
        let stmt = Query::select()
            .distinct()
            .column((Seasons::Table, Seasons::Year))
            .column((Seasons::Table, Seasons::Url))
            .from(Seasons::Table)
            .order_by((Seasons::Table, Seasons::Year), sea_query::Order::Asc)
            .to_owned();

        Self { stmt, params }.build()
    }

    fn build(self) -> Paginated<SeasonModel> {
        let page: u64 = self.params.page.unwrap_or_default();
        let limit: u64 = self.params.limit.unwrap_or_default();

        self.from(|s| s.params.driver_ref.is_some(), Drivers::Table)
            .from(|s| s.params.constructor_ref.is_some(), Constructors::Table)
            .from_clause_one()
            .from_clause_two()
            .and_where_clause_one()
            .and_where_clause_two()
            .stmt
            .paginate(page)
            .per_page(limit)
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_clause_one(self) -> Self {
        if !one_of!(
            self.params.driver_standing,
            self.params.constructor_standing
        ) {
            return self;
        }

        self.from(|_| true, Races::Table)
            .from(
                |s| one_of!(s.params.driver_standing, s.params.driver_ref),
                DriverStandings::Table,
            )
            .from(
                |s| one_of!(s.params.constructor_standing, s.params.constructor_ref),
                ConstructorStandings::Table,
            )
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_clause_two(self) -> Self {
        if one_of!(
            self.params.driver_standing,
            self.params.constructor_standing
        ) {
            return self;
        }

        self.from(
            |s| {
                one_of!(
                    s.params.circuit_ref,
                    s.params.driver_ref,
                    s.params.constructor_ref,
                    s.params.status,
                    s.params.grid,
                    s.params.fastest,
                    s.params.result
                )
            },
            Races::Table,
        )
        .from(
            |s| {
                one_of!(
                    s.params.driver_ref,
                    s.params.constructor_ref,
                    s.params.status,
                    s.params.result,
                    s.params.grid,
                    s.params.fastest
                )
            },
            Results::Table,
        )
        .from(|s| s.params.circuit_ref.is_some(), Circuits::Table)
    }

    fn and_where_clause_one(self) -> Self {
        if !one_of!(
            self.params.driver_standing,
            self.params.constructor_standing
        ) {
            return self;
        }

        self.and_where(|_| {
            Some(Expr::col((Seasons::Table, Seasons::Year)).equals((Races::Table, Races::Year)))
        })
        .and_where(|s| {
            one_of!(s.params.constructor_standing, s.params.constructor_ref).then_some(
                Expr::col((ConstructorStandings::Table, ConstructorStandings::RaceId))
                    .equals((Races::Table, Races::RaceId)),
            )
        })
        .and_where(|s| {
            s.params.constructor_ref.as_ref().map(|c| {
                Expr::col((
                    ConstructorStandings::Table,
                    ConstructorStandings::ConstructorId,
                ))
                .equals((Constructors::Table, Constructors::ConstructorId))
                .and(
                    Expr::col((Constructors::Table, Constructors::ConstructorRef))
                        .eq(Expr::value(&**c)),
                )
            })
        })
        .and_where(|s| {
            s.params.constructor_standing.map(|c| {
                Expr::col((
                    ConstructorStandings::Table,
                    ConstructorStandings::PositionText,
                ))
                .eq(Expr::value(c))
            })
        })
        .and_where(|s| {
            one_of!(s.params.driver_standing, s.params.driver_ref).then_some(
                Expr::col((DriverStandings::Table, DriverStandings::RaceId))
                    .equals((Races::Table, Races::RaceId)),
            )
        })
        .and_where(|s| {
            s.params.driver_ref.as_ref().map(|d| {
                Expr::col((DriverStandings::Table, DriverStandings::DriverId))
                    .equals((Drivers::Table, Drivers::DriverId))
                    .and(Expr::col((Drivers::Table, Drivers::DriverRef)).eq(Expr::value(&**d)))
            })
        })
        .and_where(|s| {
            s.params.driver_standing.map(|d| {
                Expr::col((DriverStandings::Table, DriverStandings::PositionText))
                    .eq(Expr::value(d))
            })
        })
        .and_where(|_| {
            Some(
                Expr::tuple(
                    [
                        Expr::col((Races::Table, Races::Year)),
                        Expr::col((Races::Table, Races::Round)),
                    ]
                    .map(Into::into),
                )
                .in_subquery(
                    Query::select()
                        .column(Races::Year)
                        .expr(Func::max(Expr::col(Races::Round)))
                        .from(Races::Table)
                        .group_by_col(Races::Year)
                        .to_owned(),
                ),
            )
        })
    }

    fn and_where_clause_two(self) -> Self {
        if one_of!(
            self.params.driver_standing,
            self.params.constructor_standing
        ) {
            return self;
        }

        self.and_where(|s| {
            one_of!(
                s.params.circuit_ref,
                s.params.driver_ref,
                s.params.constructor_ref,
                s.params.status,
                s.params.grid,
                s.params.fastest,
                s.params.result
            )
            .then_some(
                Expr::col((Seasons::Table, Seasons::Year)).equals((Races::Table, Races::Year)),
            )
        })
        .and_where(|s| {
            s.params.circuit_ref.as_ref().map(|c| {
                Expr::col((Races::Table, Races::CircuitId))
                    .equals((Circuits::Table, Circuits::CircuitId))
                    .and(Expr::col((Circuits::Table, Circuits::CircuitRef)).eq(Expr::value(&**c)))
            })
        })
        .and_where(|s| {
            one_of!(
                s.params.driver_ref,
                s.params.constructor_ref,
                s.params.status,
                s.params.grid,
                s.params.fastest,
                s.params.result
            )
            .then_some(
                Expr::col((Races::Table, Races::RaceId)).equals((Results::Table, Results::RaceId)),
            )
        })
        .and_where(|s| {
            s.params.constructor_ref.as_ref().map(|c| {
                Expr::col((Results::Table, Results::ConstructorId))
                    .equals((Constructors::Table, Constructors::ConstructorId))
                    .and(
                        Expr::col((Constructors::Table, Constructors::ConstructorRef))
                            .eq(Expr::value(&**c)),
                    )
            })
        })
        .and_where(|s| {
            s.params.driver_ref.as_ref().map(|d| {
                Expr::col((Results::Table, Results::DriverId))
                    .equals((Drivers::Table, Drivers::DriverId))
                    .and(Expr::col((Drivers::Table, Drivers::DriverRef)).eq(Expr::value(&**d)))
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
                .result
                .map(|r| Expr::col((Results::Table, Results::PositionText)).eq(Expr::value(r)))
        })
        .and_where(|s| {
            s.params
                .fastest
                .map(|f| Expr::col((Results::Table, Results::Rank)).eq(Expr::value(f)))
        })
    }
}

impl SqlBuilder for SeasonsQueryBuilder {
    fn stmt(&mut self) -> &mut sea_query::SelectStatement {
        &mut self.stmt
    }
}
