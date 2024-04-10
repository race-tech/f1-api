use sea_query::{Expr, Query, SelectStatement};

use crate::{
    iden::*,
    pagination::{Paginate, Paginated},
};
use shared::filters::GetCircuitsFilter;

pub struct CircuitQueryBuilder {
    stmt: SelectStatement,
    filter: GetCircuitsFilter,
}

impl CircuitQueryBuilder {
    pub fn filter(filter: GetCircuitsFilter) -> Self {
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
            .to_owned();

        Self { stmt, filter }
    }

    pub fn build(self) -> Paginated {
        let page: u64 = self.filter.page.unwrap_or_default().0;
        let limit: u64 = self.filter.limit.unwrap_or_default().0;

        self.races_table()
            .results_table()
            .constructors_table()
            .drivers_table()
            .stmt
            .paginate(page)
            .per_page(limit)
    }

    fn one_of(&self) -> bool {
        self.filter.driver_ref.is_some()
            || self.filter.constructor_ref.is_some()
            || self.filter.circuit_ref.is_some()
            || self.filter.grid.is_some()
            || self.filter.result.is_some()
            || self.filter.year.is_some()
            || self.filter.round.is_some()
    }

    fn races_table(mut self) -> Self {
        if self.one_of() {
            self.stmt.join(
                sea_query::JoinType::Join,
                Races::Table,
                Expr::col((Circuits::Table, Circuits::CircuitId))
                    .equals((Races::Table, Races::CircuitId)),
            );
        }
        self
    }

    fn results_table(mut self) -> Self {
        if self.one_of() {
            self.stmt.join(
                sea_query::JoinType::Join,
                Results::Table,
                Expr::col((Races::Table, Races::CircuitId))
                    .equals((Results::Table, Results::RaceId)),
            );
        }
        self
    }

    fn drivers_table(mut self) -> Self {
        if self.filter.driver_ref.is_some() {
            self.stmt.join(
                sea_query::JoinType::Join,
                Drivers::Table,
                Expr::col((Results::Table, Results::DriverId))
                    .equals((Drivers::Table, Drivers::DriverId)),
            );
        }
        self
    }

    fn constructors_table(mut self) -> Self {
        if self.filter.constructor_ref.is_some() {
            self.stmt.join(
                sea_query::JoinType::Join,
                Constructors::Table,
                Expr::col((Results::Table, Results::ConstructorId))
                    .equals((Constructors::Table, Constructors::ConstructorId)),
            );
        }
        self
    }
}
