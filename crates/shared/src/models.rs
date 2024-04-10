use rocket::serde::Serialize;

#[derive(Debug, Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Pagination {
    pub limit: u64,
    pub page: u64,
    pub max_page: u64,
    pub total: u64,
}

#[derive(Debug, Default, Serialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub enum Series {
    #[default]
    #[serde(rename = "f1")]
    F1,
    #[serde(rename = "f2")]
    F2,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Driver {
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

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Constructor {
    pub constructor_ref: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Season {
    pub year: i32,
    pub url: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DriverStanding {
    pub driver: Driver,
    pub points: f32,
    pub wins: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_text: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ConstructorStanding {
    pub constructor: Constructor,
    pub points: f32,
    pub wins: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_text: Option<String>,
    pub season: i32,
    pub round: i32,
}
