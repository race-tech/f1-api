use sea_query::{Expr, Query, SelectStatement, SimpleExpr};

use crate::{
    iden::*,
    pagination::{Paginate, Paginated},
    sql::*,
};
use shared::parameters::GetCircuitsParameter;

pub struct CircuitQueryBuilder {
    stmt: SelectStatement,
    params: GetCircuitsParameter,
}

impl CircuitQueryBuilder {
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
            .to_owned();

        Self { stmt, params }
    }

    pub fn build(self) -> Paginated {
        let page: u64 = self.params.page.unwrap_or_default().0;
        let limit: u64 = self.params.limit.unwrap_or_default().0;

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
        self.params.driver_ref.is_some()
            || self.params.constructor_ref.is_some()
            || self.params.status.is_some()
            || self.params.grid.is_some()
            || self.params.fastest.is_some()
            || self.params.result.is_some()
            || self.params.year.is_some()
            || self.params.round.is_some()
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
        self.params.driver_ref.as_ref().map(|d| Expr::value(&**d))
    }

    fn check_and_constructors(&self) -> Option<SimpleExpr> {
        self.params
            .constructor_ref
            .as_ref()
            .map(|c| Expr::value(&**c))
    }

    fn check_and_status(&self) -> Option<SimpleExpr> {
        self.params.status.as_ref().map(|s| Expr::value(**s))
    }

    fn check_and_grid(&self) -> Option<SimpleExpr> {
        self.params.grid.as_ref().map(|g| Expr::value(**g))
    }

    fn check_and_result(&self) -> Option<SimpleExpr> {
        self.params.result.as_ref().map(|r| Expr::value(**r))
    }

    fn check_and_round(&self) -> Option<SimpleExpr> {
        self.params.round.as_ref().map(|r| Expr::value(**r))
    }

    fn check_and_year(&self) -> Option<SimpleExpr> {
        self.params.year.as_ref().map(|y| Expr::value(**y))
    }

    fn check_and_fastest(&self) -> Option<SimpleExpr> {
        self.params.fastest.as_ref().map(|f| Expr::value(**f))
    }
}
