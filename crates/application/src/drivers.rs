use sea_query::{Expr, Func, Query, SelectStatement};

use shared::models::Driver as DriversModel;
use shared::parameters::GetDriversParameter;

use crate::{
    iden::*,
    one_of,
    pagination::{Paginate, Paginated},
    sql::SqlBuilder,
};

pub struct DriversQueryBuilder {
    stmt: SelectStatement,
    params: GetDriversParameter,
}

impl DriversQueryBuilder {
    pub fn params(params: GetDriversParameter) -> Self {
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

        Self { stmt, params }
    }

    fn one_of(&self) -> bool {
        self.params.year.is_some()
            || self.params.constructor_ref.is_some()
            || self.params.status.is_some()
            || self.params.grid.is_some()
            || self.params.result.is_some()
            || self.params.circuit_ref.is_some()
            || self.params.fastest.is_some()
    }

    pub fn build(self) -> Paginated<DriversModel> {
        let page: u64 = self.params.page.unwrap_or_default().0;
        let limit: u64 = self.params.limit.unwrap_or_default().0;

        self.from(
            |s| {
                one_of!(
                    s.params.year,
                    s.params.circuit_ref,
                    s.params.driver_standing
                )
            },
            Races::Table,
        )
        .from(Self::one_of, Results::Table)
        .from(|s| s.params.circuit_ref.is_some(), Circuits::Table)
        .from(|s| s.params.constructor_ref.is_some(), Constructors::Table)
        .from(
            |s| s.params.driver_standing.is_some(),
            DriverStandings::Table,
        )
        .and_clause_one()
        .and_clause_two()
        .and_where(|s| {
            s.params
                .year
                .map(|year| Expr::col((Races::Table, Races::Year)).eq(Expr::value(*year)))
        })
        .and_clause_three()
        .stmt
        .to_owned()
        .paginate(page)
        .per_page(limit)
    }

    fn and_clause_one(self) -> Self {
        if self.params.driver_standing.is_none() {
            return self;
        }

        self.and_where(|s| {
            if one_of!(s.params.year, s.params.constructor_ref) {
                Some(
                    Expr::col((Drivers::Table, Drivers::DriverId))
                        .equals((Results::Table, Results::DriverId)),
                )
            } else {
                None
            }
        })
        .and_where(|s| {
            s.params.year.map(|_| {
                Expr::col((Results::Table, Results::RaceId)).equals((Races::Table, Races::RaceId))
            })
        })
        .and_where(|s| {
            s.params.constructor_ref.as_ref().map(|constructor_ref| {
                Expr::col((Results::Table, Results::ConstructorId))
                    .equals((Constructors::Table, Constructors::ConstructorId))
                    .and(
                        Expr::col((Constructors::Table, Constructors::ConstructorRef))
                            .eq(Expr::value(&**constructor_ref)),
                    )
            })
        })
        .and_where(|s| {
            s.params.driver_standing.as_ref().map(|driver_standings| {
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

    fn and_clause_two(self) -> Self {
        if self.params.driver_standing.is_some() {
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
            if s.params.year.is_some() || s.params.circuit_ref.is_some() {
                Some(
                    Expr::col((Results::Table, Results::RaceId))
                        .equals((Races::Table, Races::RaceId)),
                )
            } else {
                None
            }
        })
        .and_where(|s| {
            s.params.circuit_ref.as_ref().map(|circuit_ref| {
                Expr::col((Races::Table, Races::CircuitId))
                    .equals((Circuits::Table, Circuits::CircuitId))
                    .and(
                        Expr::col((Circuits::Table, Circuits::CircuitRef))
                            .eq(Expr::value(&**circuit_ref)),
                    )
            })
        })
        .and_where(|s| {
            s.params.constructor_ref.as_ref().map(|constructor_ref| {
                Expr::col((Results::Table, Results::ConstructorId))
                    .equals((Constructors::Table, Constructors::ConstructorId))
                    .and(
                        Expr::col((Constructors::Table, Constructors::ConstructorRef))
                            .eq(Expr::value(&**constructor_ref)),
                    )
            })
        })
        .and_where(|s| {
            s.params.status.map(|status| {
                Expr::col((Results::Table, Results::StatusId)).eq(Expr::value(*status))
            })
        })
        .and_where(|s| {
            s.params
                .grid
                .map(|grid| Expr::col((Results::Table, Results::Grid)).eq(Expr::value(*grid)))
        })
        .and_where(|s| {
            s.params
                .fastest
                .map(|fastest| Expr::col((Results::Table, Results::Rank)).eq(Expr::value(*fastest)))
        })
        .and_where(|s| {
            s.params.result.map(|result| {
                Expr::col((Results::Table, Results::PositionText)).eq(Expr::value(*result))
            })
        })
    }

    fn and_clause_three(self) -> Self {
        if let Some(round) = self.params.round.map(|r| *r) {
            return self.and_where(|_| {
                Some(Expr::col((Races::Table, Races::Round)).eq(Expr::value(round)))
            });
        }

        if self.params.driver_standing.is_some() {
            if let Some(year) = self.params.year.map(|y| *y) {
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
}

impl SqlBuilder for DriversQueryBuilder {
    fn stmt(&mut self) -> &mut sea_query::SelectStatement {
        &mut self.stmt
    }
}
