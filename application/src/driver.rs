use diesel::dsl::{AsSelect, Select};
use diesel::prelude::*;
use diesel::{Identifiable, Queryable, Selectable};

use super::schema::drivers;

#[derive(Identifiable, Queryable, Selectable, Debug, serde::Serialize)]
#[diesel(primary_key(driver_id))]
#[diesel(table_name = drivers, check_for_backend(super::Backend))]
pub struct Driver {
    pub driver_id: i32,
    pub driver_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "permanent_number")]
    pub number: Option<i32>,
    pub code: Option<String>,
    #[serde(rename = "given_name")]
    pub forename: String,
    #[serde(rename = "family_name")]
    pub surname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "date_of_birth")]
    pub dob: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    pub url: String,
}

type All = Select<drivers::table, AsSelect<Driver, super::Backend>>;
type Limit = diesel::dsl::Limit<All>;

impl Driver {
    pub fn all() -> All {
        drivers::table.select(Driver::as_select())
    }

    pub fn limit(limit: i64) -> Limit {
        Self::all().limit(limit)
    }
}