use sea_query::{Query, SelectStatement};

use crate::{
    iden::*,
    pagination::{Paginate, Paginated},
    sql::*,
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
            .and_races()
            .and_results()
            .and_drivers()
            .and_constructors()
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

    fn races_table(self) -> Self {
        races_table(self, Self::one_of)
    }

    fn results_table(self) -> Self {
        results_table(self, Self::one_of)
    }

    fn drivers_table(self) -> Self {
        drivers_table(self, |b| b.filter.driver_ref.is_some())
    }

    fn constructors_table(self) -> Self {
        constructors_table(self, |s| s.filter.constructor_ref.is_some())
    }

    fn and_races(self) -> Self {
        and_races(self, Self::one_of)
    }

    fn and_results(self) -> Self {
        and_results(self, Self::one_of)
    }

    fn and_drivers(self) -> Self {
        and_drivers(self, |b| b.filter.driver_ref.as_ref().map(|d| d.0.clone()))
    }

    fn and_constructors(self) -> Self {
        and_constructors(self, |b| {
            b.filter.constructor_ref.as_ref().map(|d| d.0.clone())
        })
    }
}

impl SqlBuilder for CircuitQueryBuilder {
    fn stmt(&mut self) -> &mut sea_query::SelectStatement {
        &mut self.stmt
    }
}
