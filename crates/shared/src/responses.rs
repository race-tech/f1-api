use serde::Serialize;

use crate::parameters::Series;

#[derive(Debug, Serialize, Default)]
pub struct Pagination {
    pub limit: u64,
    pub page: u64,
    pub max_page: u64,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct Response<T> {
    pub data: T,

    #[serde(flatten)]
    pub pagination: Pagination,
    pub series: Series,
}

impl<T> Response<Vec<T>> {
    pub fn new<U: Into<T>>(data: Vec<U>, pagination: Pagination, series: Series) -> Self {
        Response {
            data: data.into_iter().map(Into::into).collect(),
            pagination,
            series,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RaceResponse {
    #[serde(flatten)]
    race: Race,

    circuit: Circuit,
}

#[derive(Debug, Serialize)]
pub struct ConstructorStandingResponse {
    #[serde(flatten)]
    standing: Standing,

    constructor: Constructor,
}

#[derive(Debug, Serialize)]
pub struct DriverStandingResponse {
    #[serde(flatten)]
    standing: Standing,

    driver: Driver,
}

#[derive(Debug, Serialize)]
pub struct LapsResponse {
    url: Option<String>,
    race_name: String,
    date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<chrono::NaiveTime>,

    circuit: Circuit,

    laps: Vec<Lap>,
}

#[derive(Debug, Serialize)]
pub struct PitStopsResponse {
    url: Option<String>,
    race_name: String,
    date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<chrono::NaiveTime>,

    circuit: Circuit,

    pit_stops: Vec<PitStop>,
}

#[derive(Debug, Serialize)]
pub struct Race {
    season: i32,
    round: i32,
    name: String,
    date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<chrono::NaiveTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fp1: Option<DateAndTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fp2: Option<DateAndTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fp3: Option<DateAndTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quali: Option<DateAndTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sprint: Option<DateAndTime>,
}

#[derive(Debug, Serialize)]
pub struct Circuit {
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

#[derive(Debug, Serialize)]
pub struct Driver {
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

#[derive(Debug, Serialize)]
pub struct Constructor {
    constructor_ref: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nationality: Option<String>,
    url: String,
}

#[derive(Debug, Serialize)]
pub struct Standing {
    points: f32,
    position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position_text: Option<String>,
    wins: i32,
}

#[derive(Debug, Serialize)]
pub struct DateAndTime {
    date: chrono::NaiveDate,
    time: chrono::NaiveTime,
}

#[derive(Debug, Serialize)]
pub struct Lap {
    number: i32,
    timings: Vec<LapTiming>,
}

#[derive(Debug, Serialize)]
pub struct LapTiming {
    driver_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PitStop {
    driver_ref: String,
    lap: i32,
    stop: i32,
    time: chrono::NaiveTime,
    duration: Option<String>,
}

impl From<Vec<crate::models::PitStop>> for PitStopsResponse {
    fn from(value: Vec<crate::models::PitStop>) -> Self {
        let circuit: Circuit = value.first().unwrap().into();
        let url = value.first().unwrap().race_url.clone();
        let race_name = value.first().unwrap().race_name.clone();
        let date = value.first().unwrap().race_date;
        let time = value.first().unwrap().race_time;
        let pit_stops = value.into_iter().map(Into::into).collect();

        PitStopsResponse {
            circuit,
            url,
            race_name,
            date,
            time,
            pit_stops,
        }
    }
}

impl From<Vec<crate::models::Lap>> for LapsResponse {
    fn from(value: Vec<crate::models::Lap>) -> Self {
        let circuit: Circuit = value.first().unwrap().into();
        let url = value.first().unwrap().race_url.clone();
        let race_name = value.first().unwrap().race_name.clone();
        let date = value.first().unwrap().race_date;
        let time = value.first().unwrap().race_time;

        let mut curr_lap_number = -1;
        let mut laps = Vec::new();

        for lap in value {
            if lap.lap != curr_lap_number {
                curr_lap_number = lap.lap;
                laps.push(Lap {
                    number: lap.lap,
                    timings: Vec::new(),
                });
            }

            laps.last_mut().unwrap().timings.push(LapTiming {
                driver_ref: lap.driver_ref.clone(),
                position: lap.position,
                time: lap.time.clone(),
            });
        }

        Self {
            url,
            race_name,
            date,
            time,
            circuit,
            laps,
        }
    }
}

impl From<crate::models::PitStop> for PitStop {
    fn from(value: crate::models::PitStop) -> Self {
        Self {
            driver_ref: value.driver_ref,
            lap: value.lap,
            stop: value.stop,
            time: value.time,
            duration: value.duration,
        }
    }
}

impl From<&crate::models::PitStop> for Circuit {
    fn from(value: &crate::models::PitStop) -> Self {
        Self {
            circuit_ref: value.circuit_ref.clone(),
            name: value.circuit_name.clone(),
            location: value.circuit_location.clone(),
            country: value.circuit_country.clone(),
            lat: value.circuit_lat,
            lng: value.circuit_lng,
            alt: value.circuit_alt,
            url: value.circuit_url.clone(),
        }
    }
}

impl From<&crate::models::Lap> for Circuit {
    fn from(value: &crate::models::Lap) -> Self {
        Self {
            circuit_ref: value.circuit_ref.clone(),
            name: value.circuit_name.clone(),
            location: value.circuit_location.clone(),
            country: value.circuit_country.clone(),
            lat: value.circuit_lat,
            lng: value.circuit_lng,
            alt: value.circuit_alt,
            url: value.circuit_url.clone(),
        }
    }
}

impl From<crate::models::DriverStanding> for DriverStandingResponse {
    fn from(value: crate::models::DriverStanding) -> Self {
        Self {
            standing: Standing {
                points: value.points,
                position: value.position,
                position_text: value.position_text.clone(),
                wins: value.wins,
            },
            driver: value.into(),
        }
    }
}

impl From<crate::models::DriverStanding> for Driver {
    fn from(value: crate::models::DriverStanding) -> Self {
        Self {
            driver_ref: value.driver_ref,
            number: value.number,
            code: value.code,
            forename: value.forename,
            surname: value.surname,
            dob: value.dob,
            nationality: value.nationality,
            url: value.url,
        }
    }
}

impl From<crate::models::ConstructorStanding> for ConstructorStandingResponse {
    fn from(value: crate::models::ConstructorStanding) -> Self {
        Self {
            standing: Standing {
                points: value.points,
                position: value.position,
                position_text: value.position_text.clone(),
                wins: value.wins,
            },
            constructor: value.into(),
        }
    }
}

impl From<crate::models::ConstructorStanding> for Constructor {
    fn from(value: crate::models::ConstructorStanding) -> Self {
        Self {
            constructor_ref: value.constructor_ref,
            name: value.name,
            nationality: value.nationality,
            url: value.url,
        }
    }
}

impl From<crate::models::Constructor> for Constructor {
    fn from(value: crate::models::Constructor) -> Self {
        Self {
            constructor_ref: value.constructor_ref,
            name: value.name,
            nationality: value.nationality,
            url: value.url,
        }
    }
}

impl From<crate::models::Driver> for Driver {
    fn from(value: crate::models::Driver) -> Self {
        Self {
            driver_ref: value.driver_ref,
            number: value.number,
            code: value.code,
            forename: value.forename,
            surname: value.surname,
            dob: value.dob,
            nationality: value.nationality,
            url: value.url,
        }
    }
}

impl From<crate::models::Circuit> for Circuit {
    fn from(value: crate::models::Circuit) -> Self {
        Self {
            circuit_ref: value.circuit_ref,
            name: value.name,
            location: value.location,
            country: value.country,
            lat: value.lat,
            lng: value.lng,
            alt: value.alt,
            url: value.url,
        }
    }
}

impl From<crate::models::Race> for RaceResponse {
    fn from(value: crate::models::Race) -> Self {
        let fp1 = if value.fp1_date.is_some() && value.fp1_time.is_some() {
            Some(DateAndTime {
                date: value.fp1_date.unwrap(),
                time: value.fp1_time.unwrap(),
            })
        } else {
            None
        };

        let fp2 = if value.fp2_date.is_some() && value.fp2_time.is_some() {
            Some(DateAndTime {
                date: value.fp2_date.unwrap(),
                time: value.fp2_time.unwrap(),
            })
        } else {
            None
        };

        let fp3 = if value.fp3_date.is_some() && value.fp3_time.is_some() {
            Some(DateAndTime {
                date: value.fp3_date.unwrap(),
                time: value.fp3_time.unwrap(),
            })
        } else {
            None
        };

        let quali = if value.quali_date.is_some() && value.quali_time.is_some() {
            Some(DateAndTime {
                date: value.quali_date.unwrap(),
                time: value.quali_time.unwrap(),
            })
        } else {
            None
        };

        let sprint = if value.sprint_date.is_some() && value.sprint_time.is_some() {
            Some(DateAndTime {
                date: value.sprint_date.unwrap(),
                time: value.sprint_time.unwrap(),
            })
        } else {
            None
        };

        Self {
            race: Race {
                season: value.year,
                round: value.round,
                name: value.race_name,
                date: value.race_date,
                time: value.race_time,
                url: value.race_url,
                fp1,
                fp2,
                fp3,
                quali,
                sprint,
            },
            circuit: Circuit {
                circuit_ref: value.circuit_ref,
                name: value.name,
                location: value.location,
                country: value.country,
                lat: value.lat,
                lng: value.lng,
                alt: value.alt,
                url: value.url,
            },
        }
    }
}
