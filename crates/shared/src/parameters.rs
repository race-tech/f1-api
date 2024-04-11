#![allow(clippy::needless_update)]

use rocket::form::FromForm;
use rocket::request::FromParam;
use serde::Serialize;

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

macros::struct_parameters!(
    GetCircuitsParameter {
        driver_ref: DriverRef,
        constructor_ref: ConstructorRef,
        status: Status,
        grid: Grid,
        fastest: Fastest,
        result: RaceResult,
        year: Year,
        round: Round,
        limit: Limit,
        page: Page
    } => crate::filters::GetCircuitsFilter;
);

macros::struct_parameters!(
    GetDriversParameter {
        circuit_ref: CircuitRef,
        constructor_ref: ConstructorRef,
        driver_standing: DriverStanding,
        status: Status,
        grid: Grid,
        fastest: Fastest,
        result: RaceResult,
        year: Year,
        round: Round,
        limit: Limit,
        page: Page
    } => crate::filters::GetDriversFilter;
);

macros::struct_parameters!(
    GetConstructorsParameter {
        circuit_ref: CircuitRef,
        driver_ref: DriverRef,
        constructor_standing: ConstructorStanding,
        status: Status,
        grid: Grid,
        fastest: Fastest,
        result: RaceResult,
        year: Year,
        round: Round,
        limit: Limit,
        page: Page
    } => crate::filters::GetConstructorsFilter;
);

macros::struct_parameters!(
    GetConstructorStandingsParameter {
        constructor_ref: ConstructorRef,
        position: ConstructorStanding,
        year: Year,
        round: Round,
        limit: Limit,
        page: Page
    } => crate::filters::GetConstructorStandingsFilter;
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

    macro_rules! struct_parameters {
        ($name:ident { $($field_name:ident: $field_type:ty),* } => $filter:path; $($rest:tt)*) => {
            #[derive(Debug, FromForm)]
            pub struct $name {
                $(
                    pub $field_name: Option<$field_type>,
                )*
            }

            macros::struct_parameters!{ impl From < $filter > { $($field_name: $field_type),* } for $name }

            macros::struct_parameters!{ $($rest)* }
        };
        (impl $trait:ident < $filter:path > { $($field_name:ident: $field_type:ty),* } for $name:ident) => {
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
        };
        () => {};
    }

    pub(super) use query_parameters;
    pub(super) use struct_parameters;
}
