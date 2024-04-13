use mysql::prelude::Queryable;
use sea_query::{Alias, Expr, MysqlQueryBuilder, Query, SelectStatement};

use shared::error;
use shared::error::Result;
use shared::models::Status as StatusModel;
use shared::parameters::{GetStatusParameters, StatusId};

use crate::{
    iden::*,
    one_of,
    pagination::{Paginate, Paginated},
    sql::SqlBuilder,
};

pub struct StatusQueryBuilder {
    stmt: SelectStatement,
    params: GetStatusParameters,
}

impl StatusQueryBuilder {
    pub fn get(status_id: StatusId, conn: &mut infrastructure::Connection) -> Result<StatusModel> {
        let query = Query::select()
            .distinct()
            .column((Status::Table, Status::Id))
            .column((Status::Table, Status::Content))
            .from(Status::Table)
            .and_where(Expr::col((Status::Table, Status::Id)).eq(Expr::value(*status_id)))
            .to_string(MysqlQueryBuilder);

        conn.query_first(query)?
            .ok_or(error!(EntityNotFound => "status with id `{}` not found", status_id.0))
    }

    pub fn params(params: GetStatusParameters) -> Paginated<StatusModel> {
        let stmt = Query::select()
            .distinct()
            .column((Status::Table, Status::Id))
            .column((Status::Table, Status::Content))
            .expr_as(Expr::cust("COUNT (*)"), Alias::new("count"))
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

    fn build(self) -> Paginated<StatusModel> {
        let page: u64 = self.params.page.unwrap_or_default().0;
        let limit: u64 = self.params.limit.unwrap_or_default().0;

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
                .map(|g| Expr::col((Results::Table, Results::Grid)).eq(Expr::value(*g)))
        })
        .and_where(|s| {
            s.params
                .result
                .map(|r| Expr::col((Results::Table, Results::PositionText)).eq(Expr::value(*r)))
        })
        .and_where(|s| {
            s.params
                .fastest
                .map(|f| Expr::col((Results::Table, Results::Rank)).eq(Expr::value(*f)))
        })
        .and_where(|s| {
            s.params
                .year
                .map(|y| Expr::col((Races::Table, Races::Year)).eq(Expr::val(*y)))
        })
        .and_where(|s| {
            s.params
                .round
                .map(|r| Expr::col((Races::Table, Races::Round)).eq(Expr::val(*r)))
        })
        .stmt
        .paginate(page)
        .per_page(limit)
    }
}

impl SqlBuilder for StatusQueryBuilder {
    fn stmt(&mut self) -> &mut sea_query::SelectStatement {
        &mut self.stmt
    }
}
