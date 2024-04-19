use serde::{Deserialize, Serialize};

use crate::error;
use crate::error::Result;
use crate::parameters::Series;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Pagination {
    pub limit: u64,
    pub page: u64,
    pub max_page: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
    pub series: Series,
}

impl<T> Response<Vec<T>> {
    pub fn from_vec<U: Into<T>>(data: Vec<U>, pagination: Pagination, series: Series) -> Self {
        Response {
            data: data.into_iter().map(Into::into).collect(),
            pagination: Some(pagination),
            series,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RaceResponse {
    #[serde(flatten)]
    pub race: Race,

    pub circuit: Circuit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstructorStandingResponse(Vec<InnerStandingResponse>);

#[derive(Debug, Serialize, Deserialize)]
pub struct DriverStandingResponse(Vec<InnerStandingResponse>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InnerStandingResponse {
    Constructor {
        season: i32,
        round: i32,
        constructor_standings: Vec<Standings>,
    },
    Driver {
        season: i32,
        round: i32,
        driver_standings: Vec<Standings>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Standings {
    Constructor {
        #[serde(flatten)]
        standing: Standing,

        constructor: Constructor,
    },
    Driver {
        #[serde(flatten)]
        standing: Standing,

        driver: Driver,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LapsResponse {
    pub url: Option<String>,
    pub race_name: String,
    pub date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<chrono::NaiveTime>,

    pub circuit: Circuit,

    pub laps: Vec<Lap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PitStopsResponse {
    pub url: Option<String>,
    pub race_name: String,
    pub date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<chrono::NaiveTime>,

    pub circuit: Circuit,

    pub pit_stops: Vec<PitStop>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Race {
    pub season: i32,
    pub round: i32,
    pub name: String,
    pub date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<chrono::NaiveTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp1: Option<DateAndTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp2: Option<DateAndTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp3: Option<DateAndTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quali: Option<DateAndTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprint: Option<DateAndTime>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Circuit {
    pub circuit_ref: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<i32>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Driver {
    pub driver_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub forename: String,
    pub surname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constructor {
    pub constructor_ref: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Standing {
    pub points: f32,
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_text: Option<String>,
    pub wins: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateAndTime {
    pub date: chrono::NaiveDate,
    pub time: chrono::NaiveTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lap {
    pub number: i32,
    pub timings: Vec<LapTiming>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LapTiming {
    pub driver_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PitStop {
    pub driver_ref: String,
    pub lap: i32,
    pub stop: i32,
    pub time: chrono::NaiveTime,
    pub duration: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    pub year: i32,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status_id: i32,
    pub status: String,
    pub count: i32,
}

impl From<crate::models::Status> for Status {
    fn from(value: crate::models::Status) -> Self {
        Self {
            status_id: value.status_id,
            status: value.status,
            count: value.count,
        }
    }
}

impl From<crate::models::Season> for Season {
    fn from(value: crate::models::Season) -> Self {
        Season {
            year: value.year,
            url: value.url,
        }
    }
}

impl TryFrom<Vec<crate::models::PitStop>> for PitStopsResponse {
    type Error = crate::error::Error;

    fn try_from(value: Vec<crate::models::PitStop>) -> Result<Self> {
        let first = match value.first() {
            Some(first) => first,
            None => {
                return Err(
                    error!(EntityNotFound => "cannot find any pit stop matching the given parameters"),
                )
            }
        };

        let circuit: Circuit = first.into();
        let url = first.race_url.clone();
        let race_name = first.race_name.clone();
        let date = first.race_date;
        let time = first.race_time;
        let pit_stops = value.into_iter().map(Into::into).collect();

        Ok(PitStopsResponse {
            circuit,
            url,
            race_name,
            date,
            time,
            pit_stops,
        })
    }
}

impl TryFrom<Vec<crate::models::Lap>> for LapsResponse {
    type Error = crate::error::Error;

    fn try_from(value: Vec<crate::models::Lap>) -> Result<Self> {
        let first = match value.first() {
            Some(first) => first,
            None => {
                return Err(
                    error!(EntityNotFound => "cannot find any laps matching the given parameters"),
                )
            }
        };

        let circuit: Circuit = first.into();
        let url = first.race_url.clone();
        let race_name = first.race_name.clone();
        let date = first.race_date;
        let time = first.race_time;

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

            // SAFETY: laps contains at least one entry because
            // `curr_lap_number` starts at -1 and `laps.number` are unsigned integers
            laps.last_mut().unwrap().timings.push(LapTiming {
                driver_ref: lap.driver_ref.clone(),
                position: lap.position,
                time: lap.time.clone(),
            });
        }

        Ok(Self {
            url,
            race_name,
            date,
            time,
            circuit,
            laps,
        })
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

impl From<&crate::models::DriverStanding> for Driver {
    fn from(value: &crate::models::DriverStanding) -> Self {
        Self {
            driver_ref: value.driver_ref.clone(),
            number: value.number,
            code: value.code.clone(),
            forename: value.forename.clone(),
            surname: value.surname.clone(),
            dob: value.dob,
            nationality: value.nationality.clone(),
            url: value.url.clone(),
        }
    }
}

impl From<&crate::models::ConstructorStanding> for Constructor {
    fn from(value: &crate::models::ConstructorStanding) -> Self {
        Self {
            constructor_ref: value.constructor_ref.clone(),
            name: value.name.clone(),
            nationality: value.nationality.clone(),
            url: value.url.clone(),
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

impl From<crate::models::Race> for RaceResponse {
    fn from(value: crate::models::Race) -> Self {
        let fp1 = match (value.fp1_date, value.fp1_time) {
            (Some(date), Some(time)) => Some(DateAndTime { date, time }),
            _ => None,
        };

        let fp2 = match (value.fp2_date, value.fp2_time) {
            (Some(date), Some(time)) => Some(DateAndTime { date, time }),
            _ => None,
        };

        let fp3 = match (value.fp3_date, value.fp3_time) {
            (Some(date), Some(time)) => Some(DateAndTime { date, time }),
            _ => None,
        };

        let quali = match (value.quali_date, value.quali_time) {
            (Some(date), Some(time)) => Some(DateAndTime { date, time }),
            _ => None,
        };

        let sprint = match (value.sprint_date, value.sprint_time) {
            (Some(date), Some(time)) => Some(DateAndTime { date, time }),
            _ => None,
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

impl From<Vec<crate::models::DriverStanding>> for DriverStandingResponse {
    fn from(value: Vec<crate::models::DriverStanding>) -> Self {
        let mut map = std::collections::BTreeMap::new();

        value.into_iter().for_each(|v| {
            let key = format!("{}-{}", v.year, v.round);
            let standing: Standings = (&v).into();

            if let InnerStandingResponse::Driver {
                driver_standings, ..
            } = map.entry(key).or_insert(InnerStandingResponse::Driver {
                season: v.year,
                round: v.round,
                driver_standings: Vec::new(),
            }) {
                driver_standings.push(standing);
            }
        });

        Self(map.into_values().collect())
    }
}

impl From<Vec<crate::models::ConstructorStanding>> for ConstructorStandingResponse {
    fn from(value: Vec<crate::models::ConstructorStanding>) -> Self {
        let mut map = std::collections::BTreeMap::new();

        value.into_iter().for_each(|v| {
            let key = format!("{}-{}", v.year, v.round);
            let standing: Standings = (&v).into();

            if let InnerStandingResponse::Constructor {
                constructor_standings,
                ..
            } = map
                .entry(key)
                .or_insert(InnerStandingResponse::Constructor {
                    season: v.year,
                    round: v.round,
                    constructor_standings: Vec::new(),
                })
            {
                constructor_standings.push(standing);
            }
        });

        Self(map.into_values().collect())
    }
}

impl From<&crate::models::ConstructorStanding> for Standing {
    fn from(value: &crate::models::ConstructorStanding) -> Self {
        Self {
            points: value.points,
            position: value.position,
            position_text: value.position_text.clone(),
            wins: value.wins,
        }
    }
}

impl From<&crate::models::ConstructorStanding> for Standings {
    fn from(value: &crate::models::ConstructorStanding) -> Self {
        Self::Constructor {
            standing: value.into(),
            constructor: value.into(),
        }
    }
}

impl From<&crate::models::DriverStanding> for Standing {
    fn from(value: &crate::models::DriverStanding) -> Self {
        Self {
            points: value.points,
            position: value.position,
            position_text: value.position_text.clone(),
            wins: value.wins,
        }
    }
}

impl From<&crate::models::DriverStanding> for Standings {
    fn from(value: &crate::models::DriverStanding) -> Self {
        Self::Driver {
            standing: value.into(),
            driver: value.into(),
        }
    }
}
