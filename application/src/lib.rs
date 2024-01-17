mod schema;
mod driver;
mod constructor_standing;
mod race;
mod driver_standing;

type Backend = diesel::mysql::Mysql;

pub mod models {
    pub use super::race::Race;
    pub use super::constructor_standing::ConstructorStanding;
    pub use super::driver_standing::DriverStanding;
    pub use super::driver::Driver;
}