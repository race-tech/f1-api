pub mod error;
pub mod models;
pub mod parameters;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::parameters::*;
}

pub const DATE_FORMAT: &[time::format_description::BorrowedFormatItem<'_>] =
    time::macros::format_description!("[year]-[month]-[day]");
pub const TIME_FORMAT: &[time::format_description::BorrowedFormatItem<'_>] =
    time::macros::format_description!("[hour]:[minute]:[second]");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_date() {
        let date = "1999-10-31";
        let parsed = time::Date::parse(date, DATE_FORMAT).unwrap();
        assert_eq!(parsed.year(), 1999);
        assert_eq!(parsed.month(), time::Month::October);
        assert_eq!(parsed.day(), 31);
    }

    #[test]
    fn test_parse_time() {
        let time = "12:00:00";
        let parsed = time::Time::parse(time, TIME_FORMAT).unwrap();
        assert_eq!(parsed.hour(), 12);
        assert_eq!(parsed.minute(), 0);
        assert_eq!(parsed.second(), 0);
    }
}
