use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Circuit {
    #[serde(rename = "ref")]
    pub circuit_ref: String,
    pub name: String,
    pub location: Option<String>,
    pub country: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub alt: Option<i32>,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Driver {
    #[serde(rename = "ref")]
    pub driver_ref: String,
    pub number: Option<i32>,
    pub code: Option<String>,
    pub forename: String,
    pub surname: String,
    pub dob: Option<time::Date>,
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Constructor {
    #[serde(rename = "ref")]
    pub constructor_ref: String,
    pub name: String,
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct ConstructorStanding {
    #[serde(rename = "constructors")]
    pub constructor: Constructor,
    pub points: f32,
    pub position: Option<i32>,
    pub position_text: Option<String>,
    pub wins: i32,
    pub race: Race,
    pub season: Season,
}

#[derive(Deserialize, Debug)]
pub struct DriverStanding {
    #[serde(rename = "ref")]
    pub driver_ref: String,
    pub number: Option<i32>,
    pub code: Option<String>,
    pub forename: String,
    pub surname: String,
    pub dob: Option<time::Date>,
    pub nationality: Option<String>,
    pub url: String,
    pub points: f32,
    pub position: Option<i32>,
    pub position_text: Option<String>,
    pub wins: i32,
    pub year: i32,
    pub round: i32,
}

#[derive(Deserialize, Debug)]
pub struct Lap {
    pub race: Race,
    pub circuit: Circuit,
    pub driver_ref: String,
    pub lap: i32,
    pub position: Option<i32>,
    pub time: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PitStop {
    pub race: Race,
    pub circuit: Circuit,
    pub driver_ref: String,
    pub stop: i32,
    pub lap: i32,
    pub time: String,
    pub duration: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Race {
    pub year: i32,
    pub round: i32,
    pub name: String,
    pub date: time::Date,
    pub time: Option<time::Time>,
    pub url: Option<String>,
    pub fp1_date: Option<time::Date>,
    pub fp1_time: Option<time::Time>,
    pub fp2_date: Option<time::Date>,
    pub fp2_time: Option<time::Time>,
    pub fp3_date: Option<time::Date>,
    pub fp3_time: Option<time::Time>,
    pub quali_date: Option<time::Date>,
    pub quali_time: Option<time::Time>,
    pub sprint_date: Option<time::Date>,
    pub sprint_time: Option<time::Time>,

    pub circuit: Circuit,
}

#[derive(Deserialize, Debug)]
pub struct Season {
    pub year: i32,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub status: String,
    pub count: i32,
}
