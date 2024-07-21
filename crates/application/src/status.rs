use sea_query::{Alias, Asterisk, Expr, Func, Query, SelectStatement};

use shared::models::graphql::GetStatusOpts;
use shared::models::Status as StatusModel;

use crate::{iden::*, one_of, sql::SqlBuilder};

pub struct StatusQueryBuilder<P> {
    stmt: SelectStatement,
    params: P,
}

impl StatusQueryBuilder<GetStatusOpts> {
    pub fn statuses(params: GetStatusOpts) -> Self {
        let stmt = Query::select()
            .distinct()
            .column((Status::Table, Status::Id))
            .column((Status::Table, Status::Content))
            .expr_as(Func::count(Expr::col(Asterisk)), Alias::new("count"))
            .from(Status::Table)
            .from(Results::Table)
            .and_where(
                Expr::col((Results::Table, Results::StatusId)).equals((Status::Table, Status::Id)),
            )
            .group_by_col((Status::Table, Status::Id))
            .order_by((Status::Table, Status::Id), sea_query::Order::Asc)
            .to_owned();

        Self { stmt, params }.build()
    }

    fn build(self) -> Self {
        self.from(
            |s| one_of!(s.params.year, s.params.round, s.params.circuit_ref),
            Races::Table,
        )
        .from(|s| s.params.driver_ref.is_some(), Drivers::Table)
        .from(|s| s.params.constructor_ref.is_some(), Constructors::Table)
        .from(|s| s.params.circuit_ref.is_some(), Circuits::Table)
        .and_where(|s| {
            one_of!(s.params.year, s.params.round, s.params.circuit_ref).then_some(
                Expr::col((Results::Table, Results::RaceId)).equals((Races::Table, Races::RaceId)),
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
            s.params.circuit_ref.as_ref().map(|c| {
                Expr::col((Races::Table, Races::CircuitId))
                    .equals((Circuits::Table, Circuits::CircuitId))
                    .and(Expr::col((Circuits::Table, Circuits::CircuitRef)).eq(Expr::value(&**c)))
            })
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
    }
}

impl<P> SqlBuilder for StatusQueryBuilder<P> {
    type Output = StatusModel;

    fn stmt(&mut self) -> &mut sea_query::SelectStatement {
        &mut self.stmt
    }
}
