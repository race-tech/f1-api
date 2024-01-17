#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::circuits)]
pub struct Circuit {
    pub circuit_id: i32,
    pub circuit_ref: String,
    pub name: String,
    pub location: Option<String>,
    pub country: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub alt: Option<i32>,
    pub url: String,
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::constructorResults)]
pub struct ConstructorResults {
    pub constructor_results_id: i32,
    pub race_id: i32,
    pub constructor_id: i32,
    pub points: Option<f32>,
    pub status: Option<String>,
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::constructors)]
pub struct Constructor {
    pub constructor_id: i32,
    pub constructor_ref: String,
    pub name: String,
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::driverStandings)]
pub struct DriverStanding {
    pub driver_standing_id: i32,
    pub race_id: i32,
    pub driver_id: i32,
    pub points: f32,
    pub position: Option<i32>,
    pub position_text: Option<String>,
    pub wins: i32,
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::drivers)]
pub struct Driver {
    pub driver_id: i32,
    pub driver_ref: String,
    pub number: Option<i32>,
    pub code: Option<i32>,
    pub forename: String,
    pub surname: String,
    pub dob: Option<chrono::NaiveDate>,
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::lapTimes)]
pub struct LapTime {
    pub race_id: i32,
    pub driver_id: i32,
    pub lap: i32,
    pub position: Option<i32>,
    pub time: Option<String>,
    pub milliseconds: Option<i32>,
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::pitStops)]
pub struct PitStop {
    pub race_id: i32,
    pub driver_id: i32,
    pub stop: i32,
    pub lap: i32,
    pub time: chrono::NaiveTime,
    pub duration: Option<String>,
    pub milliseconds: Option<i32>,
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::qualifying)]
pub struct Qualifying {
    pub qualifying_id: i32,
    pub race_id: i32,
    pub driver_id: i32,
    pub constructor_id: i32,
    pub number: i32,
    pub position: Option<i32>,
    pub q1: Option<String>,
    pub q2: Option<String>,
    pub q3: Option<String>,
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::races)]
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

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::results)]
pub struct Result {
    pub result_id: i32,
    pub race_id: i32,
    pub driver_id: i32,
    pub constructor_id: i32,
    pub number: Option<i32>,
    pub grid: i32,
    pub position: Option<i32>,
    pub position_text: String,
    pub position_order: i32,
    pub points: i32,
    pub laps: i32,
    pub time: Option<String>,
    pub milliseconds: Option<i32>,
    pub fastest_lap: Option<i32>,
    pub rank: Option<i32>,
    pub fastest_lap_time: Option<String>,
    pub fastest_lap_speed: Option<String>,
    pub status_id: i32,
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::seasons)]
pub struct Season {
    pub year: i32,
    pub url: String,
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::sprintResults)]
pub struct SprintResult {
    pub sprint_result_id: i32,
    pub race_id: i32,
    pub driver_id: i32,
    pub constructor_id: i32,
    pub number: i32,
    pub grid: i32,
    pub position: Option<i32>,
    pub position_text: String,
    pub position_order: i32,
    pub points: i32,
    pub laps: i32,
    pub time: Option<String>,
    pub milliseconds: Option<i32>,
    pub fastest_lap: Option<i32>,
    pub rank: Option<i32>,
    pub fastest_lap_time: Option<String>,
    pub fastest_lap_speed: Option<String>,
    pub status_id: i32,
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = super::schema::status)]
pub struct Status {
    pub status_id: i32,
    pub status_content: String,
}
