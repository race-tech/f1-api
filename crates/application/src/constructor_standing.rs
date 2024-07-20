use sea_query::{Expr, Func, IntoColumnRef, Query, SelectStatement};

use shared::models::ConstructorStanding as ConstructorStandingModel;
use shared::parameters::GetConstructorStandingsParameters;

use crate::{
    iden::*,
    pagination::{Paginate, Paginated},
    sql::SqlBuilder,
};

pub struct ConstructorStandingQueryBuilder {
    params: GetConstructorStandingsParameters,
    stmt: SelectStatement,
}

impl ConstructorStandingQueryBuilder {
    pub fn params(
        params: GetConstructorStandingsParameters,
    ) -> Paginated<ConstructorStandingModel> {
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
                .map(|c| (Constructors::Table, c).into_column_ref())
                .chain(
                    [
                        ConstructorStandings::Points,
                        ConstructorStandings::Position,
                        ConstructorStandings::PositionText,
                        ConstructorStandings::Wins,
                    ]
                    .into_iter()
                    .map(|c| (ConstructorStandings::Table, c).into_column_ref()),
                )
                .chain(
                    [Races::Round, Races::Year]
                        .into_iter()
                        .map(|c| (Races::Table, c).into_column_ref()),
                ),
            )
            .from(Constructors::Table)
            .from(ConstructorStandings::Table)
            .from(Races::Table)
            .and_where(
                Expr::col((ConstructorStandings::Table, ConstructorStandings::RaceId))
                    .equals((Races::Table, Races::RaceId)),
            )
            .and_where(
                Expr::col((
                    ConstructorStandings::Table,
                    ConstructorStandings::ConstructorId,
                ))
                .equals((Constructors::Table, Constructors::ConstructorId)),
            )
            .order_by((Races::Table, Races::Year), sea_query::Order::Asc)
            .order_by(
                (ConstructorStandings::Table, ConstructorStandings::Points),
                sea_query::Order::Asc,
            )
            .to_owned();

        Self { params, stmt }.build()
    }

    fn build(self) -> Paginated<ConstructorStandingModel> {
        let page: u64 = self.params.page.unwrap_or_default();
        let limit: u64 = self.params.limit.unwrap_or_default();

        self.and_where(|s| {
            s.params.position.map(|p| {
                Expr::col((
                    ConstructorStandings::Table,
                    ConstructorStandings::PositionText,
                ))
                .eq(Expr::value(p))
            })
        })
        .and_where(|s| {
            s.params.constructor_ref.as_ref().map(|c| {
                Expr::col((Constructors::Table, Constructors::ConstructorRef)).eq(Expr::value(&**c))
            })
        })
        .and_where(|s| {
            s.params
                .year
                .map(|y| Expr::col((Races::Table, Races::Year)).eq(Expr::val(y)))
        })
        .and_clause()
        .stmt
        .paginate(page)
        .per_page(limit)
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
                        .from(ConstructorStandings::Table)
                        .expr(Func::max(Expr::col(Races::Round)))
                        .and_where(
                            Expr::col((ConstructorStandings::Table, ConstructorStandings::RaceId))
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

impl SqlBuilder for ConstructorStandingQueryBuilder {
    type Output = ConstructorStandingModel;

    fn stmt(&mut self) -> &mut SelectStatement {
        &mut self.stmt
    }
}
