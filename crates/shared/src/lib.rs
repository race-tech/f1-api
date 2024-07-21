pub mod error;
pub mod models;

pub const DATE_FORMAT: &[time::format_description::BorrowedFormatItem<'_>] =
    time::macros::format_description!("[year]-[month]-[day]");
pub const TIME_FORMAT: &[time::format_description::BorrowedFormatItem<'_>] =
    time::macros::format_description!("[hour]:[minute]:[second]");
