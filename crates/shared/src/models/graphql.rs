use async_graphql::*;

#[derive(Debug, SimpleObject)]
pub struct DateAndTime {
    pub date: time::Date,
    pub time: String,
}

#[derive(InputObject)]
pub struct Pagination {
    pub limit: Option<u64>,
    pub page: Option<u64>,
}

#[derive(InputObject, Default)]
pub struct GetRacesOpts {
    pub circuit_ref: Option<String>,
    pub driver_ref: Option<String>,
    pub constructor_ref: Option<String>,
    pub status: Option<u32>,
    pub grid: Option<u32>,
    pub fastest: Option<u32>,
    pub result: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(InputObject, Default)]
pub struct GetCircuitsOpts {
    pub driver_ref: Option<String>,
    pub constructor_ref: Option<String>,
    pub status: Option<u32>,
    pub grid: Option<u32>,
    pub fastest: Option<u32>,
    pub result: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(Debug, InputObject)]
pub struct GetConstructorStandingsOpts {
    pub constructor_ref: Option<String>,
    pub position: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(Debug, SimpleObject)]
pub struct Race {
    pub season: i32,
    pub round: i32,
    pub name: String,
    pub date: time::Date,
    pub time: Option<String>,
    pub url: String,
    pub fp1: Option<DateAndTime>,
    pub fp2: Option<DateAndTime>,
    pub fp3: Option<DateAndTime>,
    pub quali: Option<DateAndTime>,
    pub sprint: Option<DateAndTime>,
}

#[derive(Debug, SimpleObject)]
pub struct Circuit {
    pub circuit_ref: String,
    pub name: String,
    pub location: Option<String>,
    pub country: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub alt: Option<i32>,
    pub url: String,
}

#[derive(Debug, SimpleObject)]
pub struct Standing {
    pub points: f32,
    pub position: Option<i32>,
    pub position_text: Option<String>,
    pub wins: i32,
}

#[derive(Debug, SimpleObject)]
pub struct Constructor {
    pub constructor_ref: String,
    pub name: String,
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(Debug, SimpleObject)]
pub struct InnerConstructorStanding {
    #[graphql(flatten)]
    pub standing: Standing,
    pub constructor: Constructor,
}

#[derive(Debug, SimpleObject)]
pub struct ConstructorStanding {
    pub season: i32,
    pub round: i32,
    pub standings: Vec<InnerConstructorStanding>,
}

#[derive(Debug, InputObject)]
pub struct GetConstructorsOpts {
    pub circuit_ref: Option<String>,
    pub driver_ref: Option<String>,
    pub constructor_standing: Option<u32>,
    pub status: Option<u32>,
    pub grid: Option<u32>,
    pub fastest: Option<u32>,
    pub result: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(Debug, InputObject)]
pub struct GetDriverStandingsOpts {
    pub driver_ref: Option<String>,
    pub position: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(Debug, SimpleObject)]
pub struct Driver {
    pub driver_ref: String,
    pub number: Option<i32>,
    pub code: Option<String>,
    pub forename: String,
    pub surname: String,
    pub dob: Option<time::Date>,
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(Debug, SimpleObject)]
pub struct InnerDriverStanding {
    #[graphql(flatten)]
    pub standing: Standing,
    pub driver: Driver,
}

#[derive(Debug, SimpleObject)]
pub struct DriverStanding {
    pub season: i32,
    pub round: i32,
    pub standings: Vec<InnerDriverStanding>,
}

#[derive(Debug, InputObject)]
pub struct GetDriversOpts {
    pub circuit_ref: Option<String>,
    pub constructor_ref: Option<String>,
    pub driver_standing: Option<u32>,
    pub status: Option<u32>,
    pub grid: Option<u32>,
    pub fastest: Option<u32>,
    pub result: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

pub struct Wrapper<T>(pub Vec<T>);

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: Some(30),
            page: Some(1),
        }
    }
}

impl From<super::Race> for Race {
    fn from(v: super::Race) -> Self {
        Race {
            season: v.year,
            round: v.round,
            name: v.name,
            date: v.race_date,
            time: v.race_time.map(|t| format!("{}", t)),
            url: v.url,
            fp1: v.fp1_date.map(|date| DateAndTime {
                date,
                time: format!("{}", v.fp1_time.unwrap()),
            }),
            fp2: v.fp2_date.map(|date| DateAndTime {
                date,
                time: format!("{}", v.fp2_time.unwrap()),
            }),
            fp3: v.fp3_date.map(|date| DateAndTime {
                date,
                time: format!("{}", v.fp3_time.unwrap()),
            }),
            quali: v.quali_date.map(|date| DateAndTime {
                date,
                time: format!("{}", v.quali_time.unwrap()),
            }),
            sprint: v.sprint_date.map(|date| DateAndTime {
                date,
                time: format!("{}", v.sprint_time.unwrap()),
            }),
        }
    }
}

impl From<super::Circuit> for Circuit {
    fn from(value: super::Circuit) -> Self {
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

impl From<&crate::models::ConstructorStanding> for InnerConstructorStanding {
    fn from(value: &crate::models::ConstructorStanding) -> Self {
        Self {
            standing: value.into(),
            constructor: value.into(),
        }
    }
}

impl From<Vec<super::ConstructorStanding>> for Wrapper<ConstructorStanding> {
    fn from(value: Vec<crate::models::ConstructorStanding>) -> Self {
        let mut map = std::collections::BTreeMap::new();

        value.into_iter().for_each(|v| {
            let key = format!("{}-{}", v.year, v.round);
            let standing: InnerConstructorStanding = (&v).into();

            let entry = map.entry(key).or_insert(ConstructorStanding {
                season: v.year,
                round: v.round,
                standings: Vec::new(),
            });

            entry.standings.push(standing);
        });

        Self(map.into_values().collect())
    }
}

impl From<super::Constructor> for Constructor {
    fn from(value: super::Constructor) -> Self {
        Self {
            constructor_ref: value.constructor_ref,
            name: value.name,
            nationality: value.nationality,
            url: value.url,
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

impl From<&crate::models::DriverStanding> for InnerDriverStanding {
    fn from(value: &crate::models::DriverStanding) -> Self {
        Self {
            standing: value.into(),
            driver: value.into(),
        }
    }
}

impl From<Vec<super::DriverStanding>> for Wrapper<DriverStanding> {
    fn from(value: Vec<crate::models::DriverStanding>) -> Self {
        let mut map = std::collections::BTreeMap::new();

        value.into_iter().for_each(|v| {
            let key = format!("{}-{}", v.year, v.round);
            let standing: InnerDriverStanding = (&v).into();

            let entry = map.entry(key).or_insert(DriverStanding {
                season: v.year,
                round: v.round,
                standings: Vec::new(),
            });

            entry.standings.push(standing);
        });

        Self(map.into_values().collect())
    }
}

impl From<super::Driver> for Driver {
    fn from(value: super::Driver) -> Self {
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
