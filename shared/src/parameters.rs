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
    #[Copy] Page(i32);
    #[Copy] Limit(i32);
    DriverRef(String) => str;
    #[Copy] DriverNumber(i32);
    ConstructorName(String) => str;
    Circuit(String) => str;
    #[Copy] Grid(i32);
    #[Copy] RaceResult(i32);
    #[Copy] ChampionshipResult(i32);
    #[Copy] Year(i32);
    #[Copy] Round(i32);
    #[Copy] DriverId(i32);
    #[Copy] RaceId(i32);
}

impl Year {
    pub fn get_current_year() -> Self {
        use chrono::Datelike;

        let now = chrono::Utc::now();
        Self(now.year())
    }
}

macros::struct_parameters!(
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

    ConstructorParameter {
        driver_ref: DriverRef,
        driver_number: DriverNumber,
        constructor: ConstructorName,
        circuit: Circuit,
        grid: Grid,
        result: RaceResult,
        limit: Limit,
        page: Page
    } => crate::filters::ConstructorFilter;

    DriverStandingParameter {
        name: DriverRef,
        result: ChampionshipResult,
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

    pub(super) use query_parameters;
    pub(super) use struct_parameters;
}
