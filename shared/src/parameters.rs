use rocket::form::FromForm;
use rocket::request::FromParam;

pub use super::models::Series;

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

pub enum Standing {
    Drivers,
    Constructors,
}

impl<'r> FromParam<'r> for Standing {
    type Error = ();

    fn from_param(param: &str) -> Result<Self, Self::Error> {
        match param {
            "drivers" => Ok(Standing::Drivers),
            "constructors" => Ok(Standing::Constructors),
            _ => Err(()),
        }
    }
}

impl<'r> FromParam<'r> for Year {
    type Error = ();

    fn from_param(param: &str) -> Result<Self, Self::Error> {
        match param.parse::<i32>() {
            Ok(year) => Ok(Year(year)),
            Err(_) => Err(()),
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

macro_rules! query_parameters {
    ($(($name:ident, $type:ty $([$($traits:ident),*])*)),*) => {
        $(
            #[derive(Debug, FromForm, Clone $($(, $traits)+)*)]
            pub struct $name(pub $type);

            impl From<$type> for $name {
                fn from(t: $type) -> Self {
                    Self(t)
                }
            }
        )*
    };
}

query_parameters!(
    (Page, i32[Copy]),
    (Limit, i32[Copy]),
    (DriverRef, String),
    (DriverNumber, i32),
    (ConstructorName, String),
    (Circuit, String),
    (Grid, i32),
    (RaceResult, i32),
    (Year, i32),
    (Round, i32),
    (DriverId, i32),
    (RaceId, i32)
);

macro_rules! struct_parameters {
    ($($name:ident { $($field_name:ident: $field_type:ty),* } => $filter:path;)*) => {
        $(
            #[derive(Debug, FromForm)]
            pub struct $name {
                $(
                    pub $field_name: Option<$field_type>,
                )*
            }

            impl From<$name> for $filter {
                fn from(p: $name) -> Self {
                    Self {
                        $(
                            $field_name: p.$field_name,
                        )*
                        ..Default::default()
                    }
                }
            }
        )*
    };
}

struct_parameters!(
    DriverParameter {
        driver_ref: DriverRef,
        driver_number: DriverNumber,
        constructor: ConstructorName,
        circuit: Circuit,
        grid: Grid,
        result: RaceResult,
        limit: Limit,
        page: Page
    } => crate::filters::DriverFilter;
    DriverStandingParameter {
        name: DriverRef,
        result: RaceResult,
        limit: Limit,
        page: Page
    } => crate::filters::DriverStandingFilter;
);

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
