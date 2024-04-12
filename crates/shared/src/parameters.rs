#![allow(clippy::needless_update)]
#![allow(clippy::blocks_in_conditions)]

use rocket::form::FromForm;
use rocket::request::FromParam;
use serde::Serialize;

use derives::FilterValidation;

#[derive(Debug, Default, Serialize, Clone, Copy)]
pub enum Series {
    #[default]
    #[serde(rename = "f1")]
    F1,
    #[serde(rename = "f2")]
    F2,
}

impl<'r> FromParam<'r> for Series {
    type Error = ();

    fn from_param(param: &str) -> Result<Self, Self::Error> {
        match param {
            "f1" => Ok(Series::F1),
            "f2" => Ok(Series::F2),
            _ => Err(()),
        }
    }
}

impl<'r> FromParam<'r> for Year {
    type Error = ();

    fn from_param(param: &str) -> Result<Self, Self::Error> {
        match param {
            "current" => Ok(Year::get_current_year()),
            _ => match param.parse::<i32>() {
                Ok(year) => Ok(Year(year)),
                Err(_) => Err(()),
            },
        }
    }
}

impl<'r> FromParam<'r> for Round {
    type Error = ();

    fn from_param(param: &str) -> Result<Self, Self::Error> {
        match param.parse::<i32>() {
            Ok(round) => Ok(Round(round)),
            Err(_) => Err(()),
        }
    }
}

macros::query_parameters! {
    #[Copy] Page(u64);
    #[Copy] Limit(u64);
    DriverRef(String) => str;
    ConstructorRef(String) => str;
    CircuitRef(String) => str;
    #[Copy] DriverStanding(i32);
    #[Copy] ConstructorStanding(i32);
    #[Copy] Grid(i32);
    #[Copy] RaceResult(i32);
    #[Copy] Year(i32);
    #[Copy] Round(i32);
    #[Copy] Fastest(i32);
    #[Copy] Status(i32);
}

impl Year {
    pub fn get_current_year() -> Self {
        use chrono::Datelike;

        let now = chrono::Utc::now();
        Self(now.year())
    }
}

pub trait FilterValidation {
    fn validate(&self) -> Result<(), crate::error::Error>;
}

#[derive(Debug, Default, FilterValidation, FromForm)]
pub struct GetCircuitsParameter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub driver_ref: Option<DriverRef>,
    pub constructor_ref: Option<ConstructorRef>,
    pub status: Option<Status>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation, FromForm)]
pub struct GetDriversParameter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub circuit_ref: Option<CircuitRef>,
    pub constructor_ref: Option<ConstructorRef>,
    pub driver_standing: Option<DriverStanding>,
    pub status: Option<Status>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation, FromForm)]
pub struct GetConstructorsParameter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub circuit_ref: Option<CircuitRef>,
    pub driver_ref: Option<DriverRef>,
    pub constructor_standing: Option<ConstructorStanding>,
    pub status: Option<Status>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation, FromForm)]
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

#[derive(Debug, Default, FilterValidation, FromForm)]
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

mod macros {
    macro_rules! query_parameters {
        ($(#[$($traits:ident),*])* $name:ident ($type:ty) => $deref:ty; $($rest:tt)*) => {
            #[derive(Debug, Clone, FromForm $($(, $traits)*)*)]
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
            #[derive(Debug, Clone, FromForm $($(, $traits)*)*)]
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
