pub mod error;
pub mod models;
pub mod parameters;
pub mod responses;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::parameters::*;
    pub use crate::responses::*;
}

pub const DATE_FORMAT: &[time::format_description::BorrowedFormatItem<'_>] =
    time::macros::format_description!("[year]-[month]-[day]");
pub const TIME_FORMAT: &[time::format_description::BorrowedFormatItem<'_>] =
    time::macros::format_description!("[hour]:[minute]:[second]");
