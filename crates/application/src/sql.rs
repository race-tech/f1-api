use sea_query::{Expr, SimpleExpr};

use crate::iden::*;

pub(crate) trait SqlBuilder: Sized {
    fn stmt(&mut self) -> &mut sea_query::SelectStatement;
    fn check_and_races(&self) -> bool;
    fn check_and_circuits(&self) -> bool;
    fn check_and_drivers(&self) -> Option<SimpleExpr>;
    fn check_and_constructors(&self) -> Option<SimpleExpr>;
    fn check_and_status(&self) -> Option<SimpleExpr>;
    fn check_and_grid(&self) -> Option<SimpleExpr>;
    fn check_and_fastest(&self) -> Option<SimpleExpr>;
    fn check_and_result(&self) -> Option<SimpleExpr>;
    fn check_and_round(&self) -> Option<SimpleExpr>;
    fn check_and_year(&self) -> Option<SimpleExpr>;

    fn check_races_table(&self) -> bool {
        self.check_and_circuits()
    }

    fn check_results_table(&self) -> bool {
        self.check_and_races()
    }

    fn check_drivers_table(&self) -> bool {
        self.check_and_drivers().is_some()
    }

    fn check_constructors_table(&self) -> bool {
        self.check_and_constructors().is_some()
    }

    fn races_table(self) -> Self {
        races_table(self, Self::check_races_table)
    }

    fn results_table(self) -> Self {
        results_table(self, Self::check_results_table)
    }

    fn drivers_table(self) -> Self {
        drivers_table(self, Self::check_drivers_table)
    }

    fn constructors_table(self) -> Self {
        constructors_table(self, Self::check_constructors_table)
    }

    fn and_circuits(self) -> Self {
        and_circuits(self, Self::check_and_circuits)
    }

    fn and_races(self) -> Self {
        and_races(self, Self::check_and_races)
    }

    fn and_drivers(self) -> Self {
        and_drivers(self, Self::check_and_drivers)
    }

    fn and_constructors(self) -> Self {
        and_constructors(self, Self::check_and_constructors)
    }

    fn and_status(self) -> Self {
        and_status(self, Self::check_and_status)
    }

    fn and_grid(self) -> Self {
        and_grid(self, Self::check_and_grid)
    }

    fn and_fastest(self) -> Self {
        and_fastest(self, Self::check_and_fastest)
    }

    fn and_result(self) -> Self {
        and_result(self, Self::check_and_result)
    }

    fn and_round(self) -> Self {
        and_round(self, Self::check_and_round)
    }

    fn and_year(self) -> Self {
        and_year(self, Self::check_and_year)
    }
}

pub(crate) fn races_table<B, F>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> bool,
{
    if check(&builder) {
        builder
            .stmt()
            .join(sea_query::JoinType::Join, Races::Table, Expr::val(1).eq(1));
    }
    builder
}

pub(crate) fn results_table<B, F>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> bool,
{
    if check(&builder) {
        builder.stmt().join(
            sea_query::JoinType::Join,
            Results::Table,
            Expr::val(1).eq(1),
        );
    }
    builder
}

pub(crate) fn drivers_table<B, F>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> bool,
{
    if check(&builder) {
        builder.stmt().join(
            sea_query::JoinType::Join,
            Drivers::Table,
            Expr::val(1).eq(1),
        );
    }
    builder
}

pub(crate) fn constructors_table<B, F>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> bool,
{
    if check(&builder) {
        builder.stmt().join(
            sea_query::JoinType::Join,
            Constructors::Table,
            Expr::val(1).eq(1),
        );
    }
    builder
}

pub(crate) fn and_circuits<B, F>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> bool,
{
    if check(&builder) {
        builder.stmt().and_where(
            Expr::col((Races::Table, Races::CircuitId))
                .equals((Circuits::Table, Circuits::CircuitId)),
        );
    }
    builder
}

pub(crate) fn and_races<B, F>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> bool,
{
    if check(&builder) {
        builder.stmt().and_where(
            Expr::col((Results::Table, Results::RaceId)).equals((Races::Table, Races::RaceId)),
        );
    }
    builder
}

pub(crate) fn and_drivers<B, F, V>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> Option<V>,
    V: Into<SimpleExpr>,
{
    if let Some(v) = check(&builder) {
        builder
            .stmt()
            .and_where(
                Expr::col((Drivers::Table, Drivers::DriverId))
                    .equals((Results::Table, Results::DriverId)),
            )
            .and_where(Expr::col((Drivers::Table, Drivers::DriverRef)).eq(v));
    }
    builder
}

pub(crate) fn and_constructors<B, F, V>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> Option<V>,
    V: Into<SimpleExpr>,
{
    if let Some(v) = check(&builder) {
        builder
            .stmt()
            .and_where(
                Expr::col((Constructors::Table, Constructors::ConstructorId))
                    .equals((Results::Table, Results::ConstructorId)),
            )
            .and_where(Expr::col((Constructors::Table, Constructors::ConstructorRef)).eq(v));
    }
    builder
}

pub(crate) fn and_status<B, F, V>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> Option<V>,
    V: Into<SimpleExpr>,
{
    if let Some(v) = check(&builder) {
        builder
            .stmt()
            .and_where(Expr::col((Results::Table, Results::Rank)).eq(v));
    }
    builder
}

pub(crate) fn and_grid<B, F, V>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> Option<V>,
    V: Into<SimpleExpr>,
{
    if let Some(v) = check(&builder) {
        builder
            .stmt()
            .and_where(Expr::col((Results::Table, Results::Grid)).eq(v));
    }
    builder
}

pub(crate) fn and_fastest<B, F, V>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> Option<V>,
    V: Into<SimpleExpr>,
{
    if let Some(v) = check(&builder) {
        builder
            .stmt()
            .and_where(Expr::col((Results::Table, Results::Rank)).eq(v));
    }
    builder
}

pub(crate) fn and_result<B, F, V>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> Option<V>,
    V: Into<SimpleExpr>,
{
    if let Some(v) = check(&builder) {
        builder
            .stmt()
            .and_where(Expr::col((Results::Table, Results::PositionText)).eq(v));
    }
    builder
}

pub(crate) fn and_round<B, F, V>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> Option<V>,
    V: Into<SimpleExpr>,
{
    if let Some(v) = check(&builder) {
        builder
            .stmt()
            .and_where(Expr::col((Races::Table, Races::Round)).eq(v));
    }
    builder
}

pub(crate) fn and_year<B, F, V>(mut builder: B, check: F) -> B
where
    B: SqlBuilder,
    F: FnOnce(&B) -> Option<V>,
    V: Into<SimpleExpr>,
{
    if let Some(v) = check(&builder) {
        builder
            .stmt()
            .and_where(Expr::col((Races::Table, Races::Year)).eq(v));
    }
    builder
}
