use serde::{Deserialize, Serialize};

use derives::FilterValidation;

use crate::models::graphql::{
    GetCircuitsOpts, GetConstructorStandingsOpts, GetConstructorsOpts, GetRacesOpts, Pagination,
};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Series {
    #[default]
    #[serde(rename = "f1")]
    F1,
    #[serde(rename = "f2")]
    F2,
}

#[derive(Debug, Default, Deserialize)]
pub struct GetRacesParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
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

#[derive(Debug, Default, Deserialize)]
pub struct GetCircuitsParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
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

#[derive(Debug, Default, Deserialize)]
pub struct GetConstructorStandingsParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
    pub constructor_ref: Option<String>,
    pub position: Option<u32>,
    pub year: Option<u32>,
    pub round: Option<u32>,
}

#[derive(Debug, Default, Deserialize)]
pub struct GetConstructorsParameters {
    pub limit: Option<u64>,
    pub page: Option<u64>,
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

impl From<(GetRacesOpts, Pagination)> for GetRacesParameters {
    fn from(value: (GetRacesOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            circuit_ref: opts.circuit_ref,
            driver_ref: opts.driver_ref,
            constructor_ref: opts.constructor_ref,
            status: opts.status,
            grid: opts.grid,
            fastest: opts.fastest,
            result: opts.result,
            year: opts.year,
            round: opts.round,
        }
    }
}

impl From<(GetCircuitsOpts, Pagination)> for GetCircuitsParameters {
    fn from(value: (GetCircuitsOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            circuit_ref: None,
            driver_ref: opts.driver_ref,
            constructor_ref: opts.constructor_ref,
            status: opts.status,
            grid: opts.grid,
            fastest: opts.fastest,
            result: opts.result,
            year: opts.year,
            round: opts.round,
        }
    }
}

impl From<(GetConstructorStandingsOpts, Pagination)> for GetConstructorStandingsParameters {
    fn from(value: (GetConstructorStandingsOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            constructor_ref: opts.constructor_ref,
            position: opts.position,
            year: opts.year,
            round: opts.round,
        }
    }
}

impl From<(GetConstructorsOpts, Pagination)> for GetConstructorsParameters {
    fn from(value: (GetConstructorsOpts, Pagination)) -> Self {
        let opts = value.0;
        let p = value.1;

        Self {
            limit: p.limit,
            page: p.page,
            circuit_ref: opts.circuit_ref,
            driver_ref: opts.driver_ref,
            constructor_standing: opts.constructor_standing,
            status: opts.status,
            grid: opts.grid,
            fastest: opts.fastest,
            result: opts.result,
            year: opts.year,
            round: opts.round,
        }
    }
}

macros::query_parameters! {
    #[Copy] Page(u64);
    #[Copy] Limit(u64);
    DriverRef(String) => str;
    ConstructorRef(String) => str;
    CircuitRef(String) => str;
    #[Copy] DriverStanding(u32);
    #[Copy] ConstructorStanding(u32);
    #[Copy] Grid(u32);
    #[Copy] RaceResult(u32);
    #[Copy] Year(u32);
    #[Copy] Round(u32);
    #[Copy] Fastest(i32);
    #[Copy] StatusId(i32);
    #[Copy] LapNumber(u32);
    #[Copy] PitStopNumber(u32);
}

impl Year {
    pub fn get_current_year() -> Self {
        let now = time::OffsetDateTime::now_utc();
        Self(now.year() as u32)
    }
}

pub trait FilterValidation {
    fn validate(&self) -> Result<(), crate::error::Error>;
}

#[derive(Debug, Default, FilterValidation, Deserialize)]
pub struct GetDriversParameter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub circuit_ref: Option<CircuitRef>,
    pub constructor_ref: Option<ConstructorRef>,
    pub driver_ref: Option<DriverRef>,
    pub driver_standing: Option<DriverStanding>,
    pub status: Option<StatusId>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation, Deserialize)]
pub struct GetDriverStandingsParameter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub driver_ref: Option<DriverRef>,
    pub position: Option<ConstructorStanding>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation, Deserialize)]
pub struct GetLapsParameter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub driver_ref: Option<DriverRef>,
    #[validation(skip)]
    pub year: Year,
    #[validation(skip)]
    pub round: Round,
    pub lap_number: Option<LapNumber>,
}

#[derive(Debug, Default, FilterValidation, Deserialize)]
pub struct GetPitStopsParameter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub driver_ref: Option<DriverRef>,
    pub year: Year,
    pub round: Round,
    pub lap_number: Option<LapNumber>,
    pub pit_stop_number: Option<PitStopNumber>,
}

#[derive(Debug, Default, FilterValidation, Deserialize)]
pub struct GetSeasonsParameters {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub season: Option<Year>,
    pub circuit_ref: Option<CircuitRef>,
    pub driver_ref: Option<DriverRef>,
    pub constructor_ref: Option<ConstructorRef>,
    pub status: Option<StatusId>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub driver_standing: Option<DriverStanding>,
    pub constructor_standing: Option<ConstructorStanding>,
}

#[derive(Debug, Default, FilterValidation, Deserialize)]
pub struct GetStatusParameters {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub circuit_ref: Option<CircuitRef>,
    pub driver_ref: Option<DriverRef>,
    pub constructor_ref: Option<ConstructorRef>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}

impl Default for Page {
    fn default() -> Self {
        Self(1)
    }
}

impl Default for Limit {
    fn default() -> Self {
        Self(30)
    }
}

impl Default for Year {
    fn default() -> Self {
        Self::get_current_year()
    }
}

impl Default for Round {
    fn default() -> Self {
        Self(1)
    }
}

mod macros {
    macro_rules! query_parameters {
        ($(#[$($traits:ident),*])* $name:ident ($type:ty) => $deref:ty; $($rest:tt)*) => {
            #[derive(Debug, Clone, Deserialize $($(, $traits)*)*)]
            pub struct $name(pub $type);

            impl From<$type> for $name {
                fn from(t: $type) -> Self {
                    Self(t)
                }
            }

            impl std::ops::Deref for $name {
                type Target = $deref;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            macros::query_parameters!{ $($rest)* }
        };
        ($(#[$($traits:ident),*])* $name:ident ($type:ty); $($rest:tt)*) => {
            #[derive(Debug, Clone, Deserialize $($(, $traits)*)*)]
            pub struct $name(pub $type);

            impl From<$type> for $name {
                fn from(t: $type) -> Self {
                    Self(t)
                }
            }

            impl std::ops::Deref for $name {
                type Target = $type;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            macros::query_parameters! { $($rest)* }
        };
        () => {};
    }

    pub(super) use query_parameters;
}
