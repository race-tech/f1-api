use serde::{Deserialize, Serialize};

use derives::FilterValidation;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Series {
    #[default]
    #[serde(rename = "f1")]
    F1,
    #[serde(rename = "f2")]
    F2,
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
        use chrono::Datelike;

        let now = chrono::Utc::now();
        Self(now.year() as u32)
    }
}

pub trait FilterValidation {
    fn validate(&self) -> Result<(), crate::error::Error>;
}

#[derive(Debug, Default, FilterValidation, Deserialize)]
pub struct GetCircuitsParameter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub circuit_ref: Option<CircuitRef>,
    pub driver_ref: Option<DriverRef>,
    pub constructor_ref: Option<ConstructorRef>,
    pub status: Option<StatusId>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
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
pub struct GetConstructorsParameter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub constructor_ref: Option<ConstructorRef>,
    pub circuit_ref: Option<CircuitRef>,
    pub driver_ref: Option<DriverRef>,
    pub constructor_standing: Option<ConstructorStanding>,
    pub status: Option<StatusId>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation, Deserialize)]
pub struct GetConstructorStandingsParameter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub constructor_ref: Option<ConstructorRef>,
    pub position: Option<ConstructorStanding>,
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
pub struct GetRacesParameters {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub circuit_ref: Option<CircuitRef>,
    pub driver_ref: Option<DriverRef>,
    pub constructor_ref: Option<ConstructorRef>,
    pub status: Option<StatusId>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
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
