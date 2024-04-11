use sea_query::{Expr, Func, IntoTableRef, Query, SelectStatement, SimpleExpr};

use shared::parameters::GetConstructorsParameter;

use crate::{
    iden::*,
    pagination::{Paginate, Paginated},
};

pub struct ConstructorQueryBuilder {
    params: GetConstructorsParameter,
    stmt: SelectStatement,
}

impl ConstructorQueryBuilder {
    pub fn params(params: GetConstructorsParameter) -> Self {
        let stmt = Query::select()
            .distinct()
            .columns(
                [
                    Constructors::ConstructorId,
                    Constructors::ConstructorRef,
                    Constructors::Name,
                    Constructors::Nationality,
                    Constructors::Url,
                ]
                .into_iter()
                .map(|c| (Constructors::Table, c)),
            )
            .from(Constructors::Table)
            .to_owned();

        Self { params, stmt }
    }

    pub fn build(mut self) -> Paginated {
        let page: u64 = self.params.page.unwrap_or_default().0;
        let limit: u64 = self.params.limit.unwrap_or_default().0;

        self.join(
            |s| {
                s.params.year.is_some()
                    || s.params.circuit_ref.is_some()
                    || s.params.constructor_standing.is_some()
            },
            Races::Table,
        )
        .join(Self::one_of, Results::Table)
        .join(|s| s.params.driver_ref.is_some(), Drivers::Table)
        .join(|s| s.params.circuit_ref.is_some(), Circuits::Table)
        .join(
            |s| s.params.constructor_standing.is_some(),
            ConstructorStandings::Table,
        )
        .and_where(|s| {
            s.one_of().then_some(
                Expr::col((Constructors::Table, Constructors::ConstructorId))
                    .equals((Results::Table, Results::ConstructorId)),
            )
        })
        .and_where(|s| {
            (s.params.year.is_some() || s.params.circuit_ref.is_some()).then_some(
                Expr::col((Results::Table, Results::RaceId)).equals((Races::Table, Races::RaceId)),
            )
        })
        .and_where(|s| {
            s.params.driver_ref.as_ref().map(|driver_ref| {
                Expr::col((Drivers::Table, Drivers::DriverId))
                    .equals((Results::Table, Results::DriverId))
                    .and(Expr::col((Drivers::Table, Drivers::DriverRef)).eq(&**driver_ref))
            })
        })
        .and_where(|s| {
            s.params
                .status
                .map(|status| Expr::col((Results::Table, Results::StatusId)).eq(*status))
        })
        .and_where(|s| {
            s.params
                .grid
                .map(|grid| Expr::col((Results::Table, Results::Grid)).eq(*grid))
        })
        .and_where(|s| {
            s.params
                .fastest
                .map(|fastest| Expr::col((Results::Table, Results::Rank)).eq(*fastest))
        })
        .and_where(|s| {
            s.params
                .result
                .map(|result| Expr::col((Results::Table, Results::PositionText)).eq(*result))
        })
        .and_where(|s| {
            s.params.constructor_standing.map(|standing| {
                Expr::col((
                    ConstructorStandings::Table,
                    ConstructorStandings::PositionText,
                ))
                .eq(*standing)
                .and(
                    Expr::col((
                        ConstructorStandings::Table,
                        ConstructorStandings::ConstructorId,
                    ))
                    .equals((Constructors::Table, Constructors::ConstructorId)),
                )
                .and(
                    Expr::col((ConstructorStandings::Table, ConstructorStandings::RaceId))
                        .equals((Races::Table, Races::RaceId)),
                )
            })
        })
        .and_where(|s| {
            s.params
                .year
                .map(|year| Expr::col((Races::Table, Races::Year)).eq(*year))
        })
        .and_clause()
        .stmt
        .to_owned()
        .paginate(page)
        .per_page(limit)
    }

    fn and_clause(&mut self) -> &mut Self {
        if let Some(round) = self.params.round {
            return self.and_where(|_| {
                Some(Expr::col((Races::Table, Races::Round)).eq(Expr::value(*round)))
            });
        }

        if self.params.constructor_standing.is_some() {
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
                            .expr(Func::max(Expr::col(Races::Round)))
                            .and_where(Expr::col((Races::Table, Races::Year)).eq(*year))
                            .to_owned(),
                    )
                },
            );

            self.and_where(|_| Some(expr));
        }

        self
    }

    fn one_of(&self) -> bool {
        self.params.year.is_some()
            || self.params.driver_ref.is_some()
            || self.params.status.is_some()
            || self.params.grid.is_some()
            || self.params.result.is_some()
            || self.params.circuit_ref.is_some()
            || self.params.fastest.is_some()
    }

    fn join<F, R>(&mut self, f: F, table: R) -> &mut Self
    where
        F: FnOnce(&Self) -> bool,
        R: IntoTableRef,
    {
        if f(self) {
            self.stmt
                .join(sea_query::JoinType::Join, table, Expr::val(1).eq(1));
        }

        self
    }

    fn and_where<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&Self) -> Option<SimpleExpr>,
    {
        if let Some(expr) = f(self) {
            self.stmt.and_where(expr);
        }

        self
    }
}
