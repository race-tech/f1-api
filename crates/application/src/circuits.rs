use sea_query::{Expr, Query, SelectStatement};

use shared::models::Circuit as CircuitsModel;
use shared::parameters::GetCircuitsParameter;

use crate::{
    iden::*,
    one_of,
    pagination::{Paginate, Paginated},
    sql::*,
};

pub struct CircuitsQueryBuilder {
    stmt: SelectStatement,
    params: GetCircuitsParameter,
}

impl CircuitsQueryBuilder {
    pub fn params(params: GetCircuitsParameter) -> Self {
        let stmt = Query::select()
            .distinct()
            .from(Circuits::Table)
            .columns(
                [
                    Circuits::CircuitId,
                    Circuits::CircuitRef,
                    Circuits::Name,
                    Circuits::Location,
                    Circuits::Country,
                    Circuits::Lat,
                    Circuits::Lng,
                    Circuits::Alt,
                    Circuits::Url,
                ]
                .into_iter()
                .map(|c| (Circuits::Table, c)),
            )
            .order_by(
                (Circuits::Table, Circuits::CircuitRef),
                sea_query::Order::Asc,
            )
            .to_owned();

        Self { stmt, params }
    }

    pub fn build(self) -> Paginated<CircuitsModel> {
        let page: u64 = self.params.page.unwrap_or_default().0;
        let limit: u64 = self.params.limit.unwrap_or_default().0;

        self.from(
            |s| {
                one_of!(
                    s.params.year,
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
                    s.params.grid,
                    s.params.fastest,
                    s.params.result
                )
            },
            Results::Table,
        )
        .from(|s| s.params.driver_ref.is_some(), Drivers::Table)
        .from(|s| s.params.constructor_ref.is_some(), Constructors::Table)
        .and_where(|s| {
            one_of!(
                s.params.year,
                s.params.driver_ref,
                s.params.constructor_ref,
                s.params.status,
                s.params.grid,
                s.params.fastest,
                s.params.result
            )
            .then_some(
                Expr::col((Circuits::Table, Circuits::CircuitId))
                    .equals((Races::Table, Races::CircuitId)),
            )
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
                Expr::col((Results::Table, Results::RaceId)).equals((Races::Table, Races::RaceId)),
            )
        })
        .and_where(|s| {
            s.params.constructor_ref.as_ref().map(|c| {
                Expr::col((Constructors::Table, Constructors::ConstructorId))
                    .equals((Results::Table, Results::ConstructorId))
                    .and(
                        Expr::col((Constructors::Table, Constructors::ConstructorRef))
                            .eq(Expr::value(&**c)),
                    )
            })
        })
        .and_where(|s| {
            s.params.driver_ref.as_ref().map(|d| {
                Expr::col((Drivers::Table, Drivers::DriverId))
                    .equals((Results::Table, Results::DriverId))
                    .and(Expr::col((Drivers::Table, Drivers::DriverRef)).eq(Expr::value(&**d)))
            })
        })
        .and_where(|s| {
            s.params
                .status
                .map(|s| Expr::col((Results::Table, Results::StatusId)).eq(Expr::value(*s)))
        })
        .and_where(|s| {
            s.params
                .grid
                .map(|g| Expr::col((Results::Table, Results::Grid)).eq(Expr::value(*g)))
        })
        .and_where(|s| {
            s.params
                .fastest
                .map(|f| Expr::col((Results::Table, Results::Rank)).eq(Expr::value(*f)))
        })
        .and_where(|s| {
            s.params
                .result
                .map(|r| Expr::col((Results::Table, Results::PositionText)).eq(Expr::value(*r)))
        })
        .and_where(|s| {
            s.params
                .round
                .map(|r| Expr::col((Races::Table, Races::Round)).eq(Expr::value(*r)))
        })
        .and_where(|s| {
            s.params
                .year
                .map(|y| Expr::col((Races::Table, Races::Year)).eq(Expr::value(*y)))
        })
        .stmt
        .paginate(page)
        .per_page(limit)
    }
}

impl SqlBuilder for CircuitsQueryBuilder {
    fn stmt(&mut self) -> &mut sea_query::SelectStatement {
        &mut self.stmt
    }
}
