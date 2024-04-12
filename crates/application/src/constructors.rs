use sea_query::{Expr, Func, Query, SelectStatement};

use shared::models::Constructors as ConstructorsModel;
use shared::parameters::GetConstructorsParameter;

use crate::{
    iden::*,
    one_of,
    pagination::{Paginate, Paginated},
    sql::SqlBuilder,
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

    pub fn build(self) -> Paginated<ConstructorsModel> {
        let page: u64 = self.params.page.unwrap_or_default().0;
        let limit: u64 = self.params.limit.unwrap_or_default().0;

        self.from(
            |s| {
                one_of!(
                    s.params.year,
                    s.params.circuit_ref,
                    s.params.constructor_standing
                )
            },
            Races::Table,
        )
        .from(Self::one_of, Results::Table)
        .from(|s| s.params.driver_ref.is_some(), Drivers::Table)
        .from(|s| s.params.circuit_ref.is_some(), Circuits::Table)
        .from(
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
            one_of!(s.params.year, s.params.circuit_ref).then_some(
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

    fn and_clause(self) -> Self {
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

            self.and_where(|_| Some(expr))
        } else {
            self
        }
    }

    fn one_of(&self) -> bool {
        one_of!(
            self.params.year,
            self.params.driver_ref,
            self.params.status,
            self.params.grid,
            self.params.result,
            self.params.circuit_ref,
            self.params.fastest
        )
    }
}

impl SqlBuilder for ConstructorQueryBuilder {
    fn stmt(&mut self) -> &mut sea_query::SelectStatement {
        &mut self.stmt
    }
}
