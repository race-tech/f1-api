use sea_query::{Expr, Func, IntoColumnRef, Query, SelectStatement};

use shared::models::graphql::GetDriverStandingsOpts;
use shared::models::DriverStanding as DriverStandingModel;

use crate::{iden::*, sql::SqlBuilder};

pub struct DriverStandingQueryBuilder<P> {
    params: P,
    stmt: SelectStatement,
}

impl DriverStandingQueryBuilder<GetDriverStandingsOpts> {
    pub fn driver_standings(params: GetDriverStandingsOpts) -> Self {
        let stmt = Query::select()
            .distinct()
            .columns(
                [
                    Drivers::DriverId,
                    Drivers::DriverRef,
                    Drivers::Number,
                    Drivers::Code,
                    Drivers::Forename,
                    Drivers::Surname,
                    Drivers::Dob,
                    Drivers::Nationality,
                    Drivers::Url,
                ]
                .into_iter()
                .map(|c| (Drivers::Table, c).into_column_ref())
                .chain(
                    [
                        DriverStandings::Points,
                        DriverStandings::Position,
                        DriverStandings::PositionText,
                        DriverStandings::Wins,
                    ]
                    .into_iter()
                    .map(|c| (DriverStandings::Table, c).into_column_ref()),
                )
                .chain(
                    [Races::Round, Races::Year]
                        .into_iter()
                        .map(|c| (Races::Table, c).into_column_ref()),
                ),
            )
            .from(Drivers::Table)
            .from(DriverStandings::Table)
            .from(Races::Table)
            .and_where(
                Expr::col((DriverStandings::Table, DriverStandings::RaceId))
                    .equals((Races::Table, Races::RaceId)),
            )
            .and_where(
                Expr::col((DriverStandings::Table, DriverStandings::DriverId))
                    .equals((Drivers::Table, Drivers::DriverId)),
            )
            .order_by((Races::Table, Races::Year), sea_query::Order::Asc)
            .order_by(
                (DriverStandings::Table, DriverStandings::Points),
                sea_query::Order::Asc,
            )
            .to_owned();

        Self { params, stmt }.build()
    }

    fn build(self) -> Self {
        self.and_where(|s| {
            s.params.position.map(|p| {
                Expr::col((DriverStandings::Table, DriverStandings::PositionText))
                    .eq(Expr::value(p))
            })
        })
        .and_where(|s| {
            s.params
                .driver_ref
                .as_ref()
                .map(|c| Expr::col((Drivers::Table, Drivers::DriverRef)).eq(Expr::value(&**c)))
        })
        .and_where(|s| {
            s.params
                .year
                .map(|y| Expr::col((Races::Table, Races::Year)).eq(Expr::val(y)))
        })
        .and_clause()
    }

    fn and_clause(self) -> Self {
        if let Some(round) = self.params.round {
            return self.and_where(|_| {
                Some(Expr::col((Races::Table, Races::Round)).eq(Expr::value(round)))
            });
        }

        let expr = self.params.year.map_or(
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
            |year| {
                Expr::col((Races::Table, Races::Round)).in_subquery(
                    Query::select()
                        .from(Races::Table)
                        .from(DriverStandings::Table)
                        .expr(Func::max(Expr::col(Races::Round)))
                        .and_where(
                            Expr::col((DriverStandings::Table, DriverStandings::RaceId))
                                .equals((Races::Table, Races::RaceId)),
                        )
                        .and_where(Expr::col((Races::Table, Races::Year)).eq(year))
                        .to_owned(),
                )
            },
        );

        self.and_where(|_| Some(expr))
    }
}

impl<P> SqlBuilder for DriverStandingQueryBuilder<P> {
    type Output = DriverStandingModel;

    fn stmt(&mut self) -> &mut SelectStatement {
        &mut self.stmt
    }
}
