use diesel::prelude::*;

use crate::schema::*;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = circuits)]
#[diesel(primary_key(circuit_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Circuit {
    circuit_id: i32,
    circuit_ref: String,
    name: String,
    location: Option<String>,
    country: Option<String>,
    lat: Option<f32>,
    lng: Option<f32>,
    alt: Option<i32>,
    url: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = constructor_results)]
#[diesel(primary_key(constructor_results_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(Race))]
#[diesel(belongs_to(Constructor))]
pub struct ConstructorResult {
    constructor_results_id: i32,
    race_id: i32,
    constructor_id: i32,
    points: Option<f32>,
    status: Option<String>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = constructor_standings)]
#[diesel(primary_key(constructor_standings_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(Race))]
#[diesel(belongs_to(Driver))]
pub struct ConstructorStanding {
    constructor_standings_id: i32,
    race_id: i32,
    constructor_id: i32,
    points: f32,
    position: Option<i32>,
    position_text: Option<String>,
    wins: i32,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = constructors)]
#[diesel(primary_key(constructor_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Constructor {
    constructor_id: i32,
    constructor_ref: String,
    name: String,
    nationality: Option<String>,
    url: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = driver_standings)]
#[diesel(primary_key(driver_standings_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(Race))]
#[diesel(belongs_to(Driver))]
pub struct DriverStanding {
    driver_standings_id: i32,
    race_id: i32,
    driver_id: i32,
    points: f32,
    position: Option<i32>,
    position_text: Option<String>,
    wins: i32,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = drivers)]
#[diesel(primary_key(driver_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Driver {
    driver_id: i32,
    driver_ref: String,
    number: Option<i32>,
    code: Option<String>,
    forename: String,
    surname: String,
    dob: Option<chrono::NaiveDate>,
    nationality: Option<String>,
    url: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = lap_times)]
#[diesel(primary_key(race_id, driver_id, lap))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(Race))]
#[diesel(belongs_to(Driver))]
pub struct LapTime {
    race_id: i32,
    driver_id: i32,
    lap: i32,
    position: Option<i32>,
    time: Option<String>,
    milliseconds: Option<i32>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = pit_stops)]
#[diesel(primary_key(race_id, driver_id, stop))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(Race))]
#[diesel(belongs_to(Driver))]
pub struct PitStop {
    race_id: i32,
    driver_id: i32,
    stop: i32,
    lap: i32,
    time: chrono::NaiveTime,
    duration: Option<String>,
    milliseconds: Option<i32>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = qualifying)]
#[diesel(primary_key(qualify_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(Race))]
#[diesel(belongs_to(Driver))]
#[diesel(belongs_to(Constructor))]
pub struct Qualifying {
    qualify_id: i32,
    race_id: i32,
    driver_id: i32,
    constructor_id: i32,
    number: i32,
    position: Option<i32>,
    q1: Option<String>,
    q2: Option<String>,
    q3: Option<String>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Associations)]
#[diesel(table_name = races)]
#[diesel(primary_key(race_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(Season, foreign_key = year))]
#[diesel(belongs_to(Circuit))]
pub struct Race {
    race_id: i32,
    year: i32,
    round: i32,
    circuit_id: i32,
    name: String,
    date: chrono::NaiveDate,
    time: Option<chrono::NaiveTime>,
    url: Option<String>,
    fp1_date: Option<chrono::NaiveDate>,
    fp1_time: Option<chrono::NaiveTime>,
    fp2_date: Option<chrono::NaiveDate>,
    fp2_time: Option<chrono::NaiveTime>,
    fp3_date: Option<chrono::NaiveDate>,
    fp3_time: Option<chrono::NaiveTime>,
    quali_date: Option<chrono::NaiveDate>,
    quali_time: Option<chrono::NaiveTime>,
    sprint_date: Option<chrono::NaiveDate>,
    sprint_time: Option<chrono::NaiveTime>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = results)]
#[diesel(primary_key(result_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(Race))]
#[diesel(belongs_to(Driver))]
#[diesel(belongs_to(Constructor))]
#[diesel(belongs_to(Status))]
pub struct Result {
    result_id: i32,
    race_id: i32,
    driver_id: i32,
    constructor_id: i32,
    number: Option<i32>,
    grid: i32,
    position: Option<i32>,
    position_text: String,
    position_order: i32,
    points: f32,
    laps: i32,
    time: Option<String>,
    milliseconds: Option<i32>,
    fastest_lap: Option<i32>,
    rank: Option<i32>,
    fastest_lap_time: Option<String>,
    fastest_lap_speed: Option<String>,
    status_id: i32,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = sprint_results)]
#[diesel(primary_key(sprint_result_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(belongs_to(Race))]
#[diesel(belongs_to(Driver))]
#[diesel(belongs_to(Constructor))]
#[diesel(belongs_to(Status))]
pub struct SprintResult {
    sprint_result_id: i32,
    race_id: i32,
    driver_id: i32,
    constructor_id: i32,
    number: Option<i32>,
    grid: i32,
    position: Option<i32>,
    position_text: String,
    position_order: i32,
    points: f32,
    laps: i32,
    time: Option<String>,
    milliseconds: Option<i32>,
    fastest_lap: Option<i32>,
    fastest_lap_time: Option<String>,
    status_id: i32,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = seasons)]
#[diesel(primary_key(year))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Season {
    year: i32,
    url: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = status)]
#[diesel(primary_key(status_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Status {
    status_id: i32,
    status_content: String,
}
