use diesel::{
    helper_types::{And, AsSelect, Desc, Eq, Filter, Order, Select},
    prelude::*,
};

use shared::parameters::{Round, Year};

use crate::models::Race;
use crate::prelude::*;

type All = Select<races::table, AsSelect<Race, super::Backend>>;
type ByYear = Filter<All, Eq<races::year, i32>>;
type ByYearAndRound = Filter<All, And<Eq<races::year, i32>, Eq<races::round, i32>>>;
type OrderByRound<S> = Order<S, Desc<races::round>>;

impl Race {
    fn all() -> All {
        races::table.select(Race::as_select())
    }

    pub fn last_race_of_year(year: Year) -> OrderByRound<ByYear> {
        Self::all()
            .filter(races::year.eq(year.0))
            .order(races::round.desc())
    }

    pub fn by_year_and_round(year: Year, round: Round) -> ByYearAndRound {
        Self::all().filter(races::year.eq(year.0).and(races::round.eq(round.0)))
    }
}
