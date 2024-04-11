use sea_query::{Expr, Query, SelectStatement, SimpleExpr};

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
            .and_status()
            .and_circuits()
            .and_races()
            .and_drivers()
            .and_constructors()
            .and_grid()
            .and_fastest()
            .and_result()
            .and_round()
            .and_year()
            .stmt
            .paginate(page)
            .per_page(limit)
    }

    fn one_of(&self) -> bool {
        self.filter.driver_ref.is_some()
            || self.filter.constructor_ref.is_some()
            || self.filter.status.is_some()
            || self.filter.grid.is_some()
            || self.filter.fastest.is_some()
            || self.filter.result.is_some()
            || self.filter.year.is_some()
            || self.filter.round.is_some()
    }
}

impl SqlBuilder for CircuitQueryBuilder {
    fn stmt(&mut self) -> &mut sea_query::SelectStatement {
        &mut self.stmt
    }

    fn check_and_circuits(&self) -> bool {
        self.one_of()
    }

    fn check_and_races(&self) -> bool {
        self.one_of()
    }

    fn check_and_drivers(&self) -> Option<SimpleExpr> {
        self.filter.driver_ref.as_ref().map(|d| Expr::value(&**d))
    }

    fn check_and_constructors(&self) -> Option<SimpleExpr> {
        self.filter
            .constructor_ref
            .as_ref()
            .map(|c| Expr::value(&**c))
    }

    fn check_and_status(&self) -> Option<SimpleExpr> {
        self.filter.status.as_ref().map(|s| Expr::value(**s))
    }

    fn check_and_grid(&self) -> Option<SimpleExpr> {
        self.filter.grid.as_ref().map(|g| Expr::value(**g))
    }

    fn check_and_result(&self) -> Option<SimpleExpr> {
        self.filter.result.as_ref().map(|r| Expr::value(**r))
    }

    fn check_and_round(&self) -> Option<SimpleExpr> {
        self.filter.round.as_ref().map(|r| Expr::value(**r))
    }

    fn check_and_year(&self) -> Option<SimpleExpr> {
        self.filter.year.as_ref().map(|y| Expr::value(**y))
    }

    fn check_and_fastest(&self) -> Option<SimpleExpr> {
        self.filter.fastest.as_ref().map(|f| Expr::value(**f))
    }
}