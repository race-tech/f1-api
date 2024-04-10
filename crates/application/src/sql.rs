use sea_query::{Expr, SimpleExpr};

use crate::iden::*;

pub(crate) trait SqlBuilder {
    fn stmt(&mut self) -> &mut sea_query::SelectStatement;
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

pub(crate) fn and_races<B, F>(mut builder: B, check: F) -> B
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

pub(crate) fn and_results<B, F>(mut builder: B, check: F) -> B
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
