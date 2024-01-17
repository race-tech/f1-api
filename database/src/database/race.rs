use diesel::dsl::{And, AsSelect, Eq, Filter, Select};
use diesel::prelude::*;
use diesel::{Identifiable, Queryable, Selectable};

use super::schema::races;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(primary_key(race_id))]
#[diesel(table_name = races, check_for_backend(super::Backend))]
pub struct Race {
    pub race_id: i32,
    pub year: i32,
    pub round: i32,
    pub circuit_id: i32,
    pub name: String,
    pub date: chrono::NaiveDate,
    pub time: Option<chrono::NaiveTime>,
    pub url: Option<String>,
    pub fp1_date: Option<chrono::NaiveDate>,
    pub fp1_time: Option<chrono::NaiveTime>,
    pub fp2_date: Option<chrono::NaiveDate>,
    pub fp2_time: Option<chrono::NaiveTime>,
    pub fp3_date: Option<chrono::NaiveDate>,
    pub fp3_time: Option<chrono::NaiveTime>,
    pub qualifying_date: Option<chrono::NaiveDate>,
    pub qualifying_time: Option<chrono::NaiveTime>,
    pub sprint_date: Option<chrono::NaiveDate>,
    pub sprint_time: Option<chrono::NaiveTime>,
}

type All = Select<races::table, AsSelect<Race, super::Backend>>;
type ByYear = Filter<All, Eq<races::year, i32>>;
type ByCircuit = Filter<All, Eq<races::circuit_id, i32>>;
type ByRound = Filter<All, Eq<races::round, i32>>;
type ByRoundAndYear = Filter<All, And<Eq<races::round, i32>, Eq<races::year, i32>>>;

impl Race {
    pub fn all() -> All {
        races::table.select(Race::as_select())
    }

    pub fn by_year(year: i32) -> ByYear {
        Self::all().filter(races::year.eq(year))
    }

    pub fn by_circuit(circuit_id: i32) -> ByCircuit {
        Self::all().filter(races::circuit_id.eq(circuit_id))
    }

    pub fn by_round(round: i32) -> ByRound {
        Self::all().filter(races::round.eq(round))
    }

    pub fn by_round_and_year(year: i32, round: i32) -> ByRoundAndYear {
        Self::all().filter(races::round.eq(round).and(races::year.eq(year)))
    }
}
