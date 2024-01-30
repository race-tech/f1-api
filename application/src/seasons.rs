use diesel::{
    helper_types::{AsSelect, Eq, Filter, Select},
    prelude::*,
};

use shared::parameters::Year;

use crate::models::Season;
use crate::prelude::*;

type All = Select<seasons::table, AsSelect<Season, super::Backend>>;
type ByYear = Filter<All, Eq<seasons::year, i32>>;

impl Season {
    fn all() -> All {
        seasons::table.select(Season::as_select())
    }

    pub fn by_year(year: Year) -> ByYear {
        Self::all().filter(seasons::year.eq(year.0))
    }
}
