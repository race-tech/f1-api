use diesel::query_builder::SqlQuery;
use diesel::QueryableByName;

use crate::{schema::circuits, sql::*};

#[derive(QueryableByName, Debug)]
#[diesel(table_name = circuits, check_for_backend(super::Backend))]
pub struct Circuit {
    #[diesel(column_name = "circuitRef")]
    circuit_ref: String,
    name: String,
    location: Option<String>,
    country: Option<String>,
    lat: Option<f32>,
    lng: Option<f32>,
    alt: Option<i32>,
    url: String,
}

const BASE_QUERY: &str = r#"SELECT DISTINCT circuits.circuitRef, circuits.name, circuits.location, circuits.country, circuits.lat, circuits.lng, circuits.alt, circuits.url FROM circuits"#;

pub fn get_circuits(params: GenericParams) -> String {
    dbg!(format!(
        "{}{} WHERE TRUE{}{}{}{}",
        BASE_QUERY,
        add_tables(&params),
        and_circuits(&params),
        and_races(&params),
        and_constructors(&params),
        and_drivers(&params)
    ))
}

fn add_tables(params: &GenericParams) -> String {
    format!(
        "{}{}{}{}",
        append_race_table(params),
        append_results_table(params),
        append_drivers_table(params),
        append_constructors_table(params)
    )
}
