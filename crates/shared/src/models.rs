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
    dob: Option<chrono::NaiveDate>,
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
    #[mysql(rename = "constructorId")]
    constructor_id: i32,
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

#[derive(FromRow, Debug, Serialize)]
pub struct DriverStandings {
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
    dob: Option<chrono::NaiveDate>,
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

#[derive(FromRow, Debug, Serialize)]
pub struct Laps {
    #[mysql(rename = "raceName")]
    race_name: String,
    #[mysql(rename = "raceDate")]
    race_date: chrono::NaiveDate,
    #[mysql(rename = "raceTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    race_time: Option<chrono::NaiveTime>,
    #[mysql(rename = "raceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    race_url: Option<String>,
    #[mysql(rename = "circuitRef")]
    circuit_ref: String,
    #[mysql(rename = "circuitName")]
    circuit_name: String,
    #[mysql(rename = "circuitLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_location: Option<String>,
    #[mysql(rename = "circuitCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_country: Option<String>,
    #[mysql(rename = "circuitLat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_lat: Option<f32>,
    #[mysql(rename = "circuitLng")]
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_lng: Option<f32>,
    #[mysql(rename = "circuitAlt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_alt: Option<i32>,
    #[mysql(rename = "circuitUrl")]
    circuit_url: String,
    #[mysql(rename = "driverRef")]
    driver_ref: String,
    lap: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<String>,
}

#[derive(FromRow, Debug, Serialize)]
pub struct PitStops {
    #[mysql(rename = "raceName")]
    race_name: String,
    #[mysql(rename = "raceDate")]
    race_date: chrono::NaiveDate,
    #[mysql(rename = "raceTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    race_time: Option<chrono::NaiveTime>,
    #[mysql(rename = "raceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    race_url: Option<String>,
    #[mysql(rename = "circuitRef")]
    circuit_ref: String,
    #[mysql(rename = "circuitName")]
    circuit_name: String,
    #[mysql(rename = "circuitLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_location: Option<String>,
    #[mysql(rename = "circuitCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_country: Option<String>,
    #[mysql(rename = "circuitLat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_lat: Option<f32>,
    #[mysql(rename = "circuitLng")]
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_lng: Option<f32>,
    #[mysql(rename = "circuitAlt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    circuit_alt: Option<i32>,
    #[mysql(rename = "circuitUrl")]
    circuit_url: String,
    #[mysql(rename = "driverRef")]
    driver_ref: String,
    stop: i32,
    lap: i32,
    time: chrono::NaiveTime,
    duration: Option<String>,
}
