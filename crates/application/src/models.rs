use mysql::prelude::*;
use serde::Serialize;

#[derive(FromRow, Debug, Serialize)]
pub struct Circuits {
    #[mysql(rename = "circuitId")]
    circuit_id: i32,
    #[mysql(rename = "circuitRef")]
    circuit_ref: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lat: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lng: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alt: Option<i32>,
    url: String,
}

#[derive(FromRow, Debug, Serialize)]
pub struct Drivers {
    #[mysql(rename = "driverId")]
    driver_id: i32,
    #[mysql(rename = "driverRef")]
    driver_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    forename: String,
    surname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    dob: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nationality: Option<String>,
    url: String,
}

#[derive(FromRow, Debug, Serialize)]
pub struct Constructors {
    #[mysql(rename = "constructorId")]
    constructor_id: i32,
    #[mysql(rename = "constructorRef")]
    constructor_ref: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nationality: Option<String>,
    url: String,
}

#[derive(FromRow, Debug, Serialize)]
pub struct ConstructorStandings {
    #[mysql(rename = "constructorRef")]
    constructor_ref: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nationality: Option<String>,
    url: String,
    points: f32,
    position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[mysql(rename = "positionText")]
    position_text: Option<String>,
    wins: i32,
    year: i32,
    round: i32,
}
