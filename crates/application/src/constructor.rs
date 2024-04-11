use sea_query::{Expr, Func, IntoTableRef, Query, SelectStatement, SimpleExpr};

use shared::filters::GetConstructorsFilter;

use crate::{
    iden::*,
    pagination::{Paginate, Paginated},
};

pub struct ConstructorQueryBuilder {
    filter: GetConstructorsFilter,
    stmt: SelectStatement,
}

impl ConstructorQueryBuilder {
    pub fn filter(filter: GetConstructorsFilter) -> Self {
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

        Self { filter, stmt }
    }

    pub fn build(mut self) -> Paginated {
        let page: u64 = self.filter.page.unwrap_or_default().0;
        let limit: u64 = self.filter.limit.unwrap_or_default().0;

        self.join(
            |s| {
                s.filter.year.is_some()
                    || s.filter.circuit_ref.is_some()
                    || s.filter.constructor_standing.is_some()
            },
            Races::Table,
        )
        .join(Self::one_of, Results::Table)
        .join(|s| s.filter.driver_ref.is_some(), Drivers::Table)
        .join(|s| s.filter.circuit_ref.is_some(), Circuits::Table)
        .join(
            |s| s.filter.constructor_standing.is_some(),
            ConstructorStandings::Table,
        )
        .and_where(|s| {
            s.one_of().then_some(
                Expr::col((Constructors::Table, Constructors::ConstructorId))
                    .equals((Results::Table, Results::ConstructorId)),
            )
        })
        .and_where(|s| {
            (s.filter.year.is_some() || s.filter.circuit_ref.is_some()).then_some(
                Expr::col((Results::Table, Results::RaceId)).equals((Races::Table, Races::RaceId)),
            )
        })
        .and_where(|s| {
            s.filter.driver_ref.as_ref().map(|driver_ref| {
                Expr::col((Drivers::Table, Drivers::DriverId))
                    .equals((Results::Table, Results::DriverId))
                    .and(Expr::col((Drivers::Table, Drivers::DriverRef)).eq(&**driver_ref))
            })
        })
        .and_where(|s| {
            s.filter
                .status
                .map(|status| Expr::col((Results::Table, Results::StatusId)).eq(*status))
        })
        .and_where(|s| {
            s.filter
                .grid
                .map(|grid| Expr::col((Results::Table, Results::Grid)).eq(*grid))
        })
        .and_where(|s| {
            s.filter
                .fastest
                .map(|fastest| Expr::col((Results::Table, Results::Rank)).eq(*fastest))
        })
        .and_where(|s| {
            s.filter
                .result
                .map(|result| Expr::col((Results::Table, Results::PositionText)).eq(*result))
        })
        .and_where(|s| {
            s.filter.constructor_standing.map(|standing| {
                Expr::col((
                    ConstructorStandings::Table,
                    ConstructorStandings::PositionText,
                ))
                .eq(*standing)
                .and(
                    Expr::col((ConstructorStandings::Table, ConstructorStandings::Id))
                        .equals((Constructors::Table, Constructors::ConstructorId)),
                )
                .and(
                    Expr::col((ConstructorStandings::Table, ConstructorStandings::RaceId))
                        .equals((Races::Table, Races::RaceId)),
                )
            })
        })
        .and_where(|s| {
            s.filter
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
        if let Some(round) = self.filter.round {
            return self.and_where(|_| {
                Some(Expr::col((Races::Table, Races::Round)).eq(Expr::value(*round)))
            });
        }

        if self.filter.constructor_standing.is_some() {
            let expr = self.filter.year.map_or(
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
        self.filter.year.is_some()
            || self.filter.driver_ref.is_some()
            || self.filter.status.is_some()
            || self.filter.grid.is_some()
            || self.filter.result.is_some()
            || self.filter.circuit_ref.is_some()
            || self.filter.fastest.is_some()
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
