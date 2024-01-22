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

macro_rules! query_parameters {
    ($(($name:ident, $type:ty $([$($traits:ident),*])*)),*) => {
        $(
            #[derive(Debug, FromForm $($(, $traits)+)*)]
            pub struct $name($type);

            impl $name {
                pub fn inner(&self) -> $type {
                    self.0.clone()
                }
            }
        )*
    };
}

query_parameters!(
    (Page, i32 [Copy, Clone]),
    (Limit, i32 [Copy, Clone]),
    (DriverRef, String),
    (DriverNumber, i32),
    (Constructor, String),
    (Circuit, String),
    (Grid, i32),
    (RaceResult, i32)
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
