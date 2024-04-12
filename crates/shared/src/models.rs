use mysql::prelude::*;

#[derive(FromRow, Debug)]
pub struct Circuit {
    #[mysql(rename = "circuitId")]
    pub circuit_id: i32,
    #[mysql(rename = "circuitRef")]
    pub circuit_ref: String,
    pub name: String,
    pub location: Option<String>,
    pub country: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub alt: Option<i32>,
    pub url: String,
}

#[derive(FromRow, Debug)]
pub struct Driver {
    #[mysql(rename = "driverId")]
    pub driver_id: i32,
    #[mysql(rename = "driverRef")]
    pub driver_ref: String,
    pub number: Option<i32>,
    pub code: Option<String>,
    pub forename: String,
    pub surname: String,
    pub dob: Option<chrono::NaiveDate>,
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(FromRow, Debug)]
pub struct Constructor {
    #[mysql(rename = "constructorId")]
    pub constructor_id: i32,
    #[mysql(rename = "constructorRef")]
    pub constructor_ref: String,
    pub name: String,
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(FromRow, Debug)]
pub struct ConstructorStanding {
    #[mysql(rename = "constructorId")]
    pub constructor_id: i32,
    #[mysql(rename = "constructorRef")]
    pub constructor_ref: String,
    pub name: String,
    pub nationality: Option<String>,
    pub url: String,
    pub points: f32,
    pub position: Option<i32>,
    #[mysql(rename = "positionText")]
    pub position_text: Option<String>,
    pub wins: i32,
    pub year: i32,
    pub round: i32,
}

#[derive(FromRow, Debug)]
pub struct DriverStanding {
    #[mysql(rename = "driverId")]
    pub driver_id: i32,
    #[mysql(rename = "driverRef")]
    pub driver_ref: String,
    pub number: Option<i32>,
    pub code: Option<String>,
    pub forename: String,
    pub surname: String,
    pub dob: Option<chrono::NaiveDate>,
    pub nationality: Option<String>,
    pub url: String,
    pub points: f32,
    pub position: Option<i32>,
    #[mysql(rename = "positionText")]
    pub position_text: Option<String>,
    pub wins: i32,
    pub year: i32,
    pub round: i32,
}

#[derive(FromRow, Debug)]
pub struct Lap {
    #[mysql(rename = "raceName")]
    pub race_name: String,
    #[mysql(rename = "raceDate")]
    pub race_date: chrono::NaiveDate,
    #[mysql(rename = "raceTime")]
    pub race_time: Option<chrono::NaiveTime>,
    #[mysql(rename = "raceUrl")]
    pub race_url: Option<String>,
    #[mysql(rename = "circuitRef")]
    pub circuit_ref: String,
    #[mysql(rename = "circuitName")]
    pub circuit_name: String,
    #[mysql(rename = "circuitLocation")]
    pub circuit_location: Option<String>,
    #[mysql(rename = "circuitCountry")]
    pub circuit_country: Option<String>,
    #[mysql(rename = "circuitLat")]
    pub circuit_lat: Option<f32>,
    #[mysql(rename = "circuitLng")]
    pub circuit_lng: Option<f32>,
    #[mysql(rename = "circuitAlt")]
    pub circuit_alt: Option<i32>,
    #[mysql(rename = "circuitUrl")]
    pub circuit_url: String,
    #[mysql(rename = "driverRef")]
    pub driver_ref: String,
    pub lap: i32,
    pub position: Option<i32>,
    pub time: Option<String>,
}

#[derive(FromRow, Debug)]
pub struct PitStop {
    #[mysql(rename = "raceName")]
    pub race_name: String,
    #[mysql(rename = "raceDate")]
    pub race_date: chrono::NaiveDate,
    #[mysql(rename = "raceTime")]
    pub race_time: Option<chrono::NaiveTime>,
    #[mysql(rename = "raceUrl")]
    pub race_url: Option<String>,
    #[mysql(rename = "circuitRef")]
    pub circuit_ref: String,
    #[mysql(rename = "circuitName")]
    pub circuit_name: String,
    #[mysql(rename = "circuitLocation")]
    pub circuit_location: Option<String>,
    #[mysql(rename = "circuitCountry")]
    pub circuit_country: Option<String>,
    #[mysql(rename = "circuitLat")]
    pub circuit_lat: Option<f32>,
    #[mysql(rename = "circuitLng")]
    pub circuit_lng: Option<f32>,
    #[mysql(rename = "circuitAlt")]
    pub circuit_alt: Option<i32>,
    #[mysql(rename = "circuitUrl")]
    pub circuit_url: String,
    #[mysql(rename = "driverRef")]
    pub driver_ref: String,
    pub stop: i32,
    pub lap: i32,
    pub time: chrono::NaiveTime,
    pub duration: Option<String>,
}

#[derive(FromRow, Debug)]
pub struct Race {
    pub year: i32,
    pub round: i32,
    #[mysql(rename = "raceName")]
    pub race_name: String,
    pub race_date: chrono::NaiveDate,
    #[mysql(rename = "time")]
    pub race_time: Option<chrono::NaiveTime>,
    #[mysql(rename = "raceUrl")]
    pub race_url: Option<String>,
    pub fp1_date: Option<chrono::NaiveDate>,
    pub fp1_time: Option<chrono::NaiveTime>,
    pub fp2_date: Option<chrono::NaiveDate>,
    pub fp2_time: Option<chrono::NaiveTime>,
    pub fp3_date: Option<chrono::NaiveDate>,
    pub fp3_time: Option<chrono::NaiveTime>,
    pub quali_date: Option<chrono::NaiveDate>,
    pub quali_time: Option<chrono::NaiveTime>,
    pub sprint_date: Option<chrono::NaiveDate>,
    pub sprint_time: Option<chrono::NaiveTime>,

    pub circuit_ref: String,
    pub name: String,
    pub location: Option<String>,
    pub country: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub alt: Option<i32>,
    pub url: String,
}

#[derive(FromRow, Debug)]
pub struct Season {
    pub year: i32,
    pub url: String,
}
