use serde::{de, Deserialize, Deserializer};

pub mod graphql;
pub mod response;
pub mod surreal;

pub use surreal::*;

fn de_option_f32<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<f32>, D::Error> {
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) if s == "None" => Ok(None),
        Some(s) => s.parse().map_err(de::Error::custom).map(Some),
        None => Ok(None),
    }
}

fn de_date<'de, D: Deserializer<'de>>(deserializer: D) -> Result<time::Date, D::Error> {
    let s = String::deserialize(deserializer)?;
    time::Date::parse(&s, crate::DATE_FORMAT).map_err(de::Error::custom)
}

fn de_option_date<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<time::Date>, D::Error> {
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) if s == "None" => Ok(None),
        Some(s) => time::Date::parse(&s, crate::DATE_FORMAT)
            .map_err(de::Error::custom)
            .map(Some),
        None => Ok(None),
    }
}

fn de_time<'de, D: Deserializer<'de>>(deserializer: D) -> Result<time::Time, D::Error> {
    let s = String::deserialize(deserializer)?;
    time::Time::parse(&s, crate::TIME_FORMAT).map_err(de::Error::custom)
}

fn de_option_time<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<time::Time>, D::Error> {
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) if s == "None" => Ok(None),
        Some(s) => time::Time::parse(&s, crate::TIME_FORMAT)
            .map_err(de::Error::custom)
            .map(Some),
        None => Ok(None),
    }
}
