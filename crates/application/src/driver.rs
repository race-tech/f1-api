use sea_query::{Expr, Func, IntoTableRef, Query, SelectStatement, SimpleExpr};

use shared::filters::GetDriversFilter;

use crate::{
    iden::*,
    pagination::{Paginate, Paginated},
};

pub struct DriverQueryBuilder {
    stmt: SelectStatement,
    filter: GetDriversFilter,
}

impl DriverQueryBuilder {
    pub fn filter(filter: GetDriversFilter) -> Self {
        let stmt = Query::select()
            .distinct()
            .from(Drivers::Table)
            .columns(
                [
                    Drivers::DriverId,
                    Drivers::DriverRef,
                    Drivers::Number,
                    Drivers::Code,
                    Drivers::Forename,
                    Drivers::Surname,
                    Drivers::Dob,
                    Drivers::Nationality,
                    Drivers::Url,
                ]
                .into_iter()
                .map(|c| (Drivers::Table, c)),
            )
            .to_owned();

        Self { stmt, filter }
    }

    fn one_of(&self) -> bool {
        self.filter.year.is_some()
            || self.filter.constructor_ref.is_some()
            || self.filter.status.is_some()
            || self.filter.grid.is_some()
            || self.filter.result.is_some()
            || self.filter.circuit_ref.is_some()
            || self.filter.fastest.is_some()
    }

    pub fn build(mut self) -> Paginated {
        let page: u64 = self.filter.page.unwrap_or_default().0;
        let limit: u64 = self.filter.limit.unwrap_or_default().0;

        self.join(
            |s| {
                s.filter.year.is_some()
                    || s.filter.circuit_ref.is_some()
                    || s.filter.driver_standing.is_some()
            },
            Races::Table,
        )
        .join(Self::one_of, Results::Table)
        .join(|s| s.filter.circuit_ref.is_some(), Circuits::Table)
        .join(|s| s.filter.constructor_ref.is_some(), Constructors::Table)
        .join(
            |s| s.filter.driver_standing.is_some(),
            DriverStandings::Table,
        )
        .and_clause_one()
        .and_clause_two()
        .and_where(|s| {
            s.filter
                .year
                .map(|year| Expr::col((Races::Table, Races::Year)).eq(Expr::value(*year)))
        })
        .and_clause_three()
        .stmt
        .to_owned()
        .paginate(page)
        .per_page(limit)
    }

    fn and_clause_one(&mut self) -> &mut Self {
        if self.filter.driver_standing.is_none() {
            return self;
        }

        self.and_where(|s| {
            if s.filter.year.is_some() || s.filter.constructor_ref.is_some() {
                Some(
                    Expr::col((Drivers::Table, Drivers::DriverId))
                        .equals((Results::Table, Results::DriverId)),
                )
            } else {
                None
            }
        })
        .and_where(|s| {
            s.filter.year.map(|_| {
                Expr::col((Results::Table, Results::RaceId)).equals((Races::Table, Races::RaceId))
            })
        })
        .and_where(|s| {
            s.filter.constructor_ref.as_ref().map(|constructor_ref| {
                Expr::col((Results::Table, Results::ConstructorId))
                    .equals((Constructors::Table, Constructors::ConstructorId))
                    .and(
                        Expr::col((Constructors::Table, Constructors::ConstructorRef))
                            .eq(Expr::value(&**constructor_ref)),
                    )
            })
        })
        .and_where(|s| {
            s.filter.driver_standing.as_ref().map(|driver_standings| {
                Expr::col((DriverStandings::Table, DriverStandings::PositionText))
                    .eq(Expr::value(**driver_standings))
            })
        })
        .and_where(|_| {
            Some(
                Expr::col((DriverStandings::Table, DriverStandings::RaceId))
                    .equals((Races::Table, Races::RaceId)),
            )
        })
        .and_where(|_| {
            Some(
                Expr::col((DriverStandings::Table, DriverStandings::DriverId))
                    .equals((Drivers::Table, Drivers::DriverId)),
            )
        })
    }

    fn and_clause_two(&mut self) -> &mut Self {
        if self.filter.driver_standing.is_some() {
            return self;
        }

        self.and_where(|s| {
            if s.one_of() {
                Some(
                    Expr::col((Drivers::Table, Drivers::DriverId))
                        .equals((Results::Table, Results::DriverId)),
                )
            } else {
                None
            }
        })
        .and_where(|s| {
            if s.filter.year.is_some() || s.filter.circuit_ref.is_some() {
                Some(
                    Expr::col((Results::Table, Results::RaceId))
                        .equals((Races::Table, Races::RaceId)),
                )
            } else {
                None
            }
        })
        .and_where(|s| {
            s.filter.circuit_ref.as_ref().map(|circuit_ref| {
                Expr::col((Races::Table, Races::CircuitId))
                    .equals((Circuits::Table, Circuits::CircuitId))
                    .and(
                        Expr::col((Circuits::Table, Circuits::CircuitRef))
                            .eq(Expr::value(&**circuit_ref)),
                    )
            })
        })
        .and_where(|s| {
            s.filter.constructor_ref.as_ref().map(|constructor_ref| {
                Expr::col((Results::Table, Results::ConstructorId))
                    .equals((Constructors::Table, Constructors::ConstructorId))
                    .and(
                        Expr::col((Constructors::Table, Constructors::ConstructorRef))
                            .eq(Expr::value(&**constructor_ref)),
                    )
            })
        })
        .and_where(|s| {
            s.filter.status.map(|status| {
                Expr::col((Results::Table, Results::StatusId)).eq(Expr::value(*status))
            })
        })
        .and_where(|s| {
            s.filter
                .grid
                .map(|grid| Expr::col((Results::Table, Results::Grid)).eq(Expr::value(*grid)))
        })
        .and_where(|s| {
            s.filter
                .fastest
                .map(|fastest| Expr::col((Results::Table, Results::Rank)).eq(Expr::value(*fastest)))
        })
        .and_where(|s| {
            s.filter.result.map(|result| {
                Expr::col((Results::Table, Results::PositionText)).eq(Expr::value(*result))
            })
        })
    }

    fn and_clause_three(&mut self) -> &mut Self {
        if let Some(round) = self.filter.round.map(|r| *r) {
            return self.and_where(|_| {
                Some(Expr::col((Races::Table, Races::Round)).eq(Expr::value(round)))
            });
        }

        if self.filter.driver_standing.is_some() {
            if let Some(year) = self.filter.year.map(|y| *y) {
                return self.and_where(|_| {
                    Some(
                        Expr::col((Races::Table, Races::Round)).in_subquery(
                            Query::select()
                                .expr(Func::max(Expr::col(Races::Round)))
                                .from(Races::Table)
                                .and_where(
                                    Expr::col((Races::Table, Races::Year)).eq(Expr::value(year)),
                                )
                                .to_owned(),
                        ),
                    )
                });
            } else {
                return self.and_where(|_| {
                    Some(
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
                    )
                });
            }
        }

        self
    }

    fn join<F, R>(&mut self, f: F, table: R) -> &mut Self
    where
        F: FnOnce(&Self) -> bool,
        R: IntoTableRef,
    {
        if f(self) {
            self.stmt
                .join(sea_query::JoinType::Join, table, Expr::val(1).eq(1));
        }

        self
    }

    fn and_where<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&Self) -> Option<SimpleExpr>,
    {
        if let Some(expr) = f(self) {
            self.stmt.and_where(expr);
        }

        self
    }
}
