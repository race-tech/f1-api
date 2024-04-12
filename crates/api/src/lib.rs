#![allow(clippy::too_many_arguments)]

mod circuits;
mod constructor_standings;
mod constructors;
mod driver_standings;
mod drivers;
mod laps;

pub mod handlers {
    use crate::*;
    use rocket::Route;

    pub fn handlers() -> Vec<Route> {
        circuits::handlers()
            .into_iter()
            .chain(drivers::handlers())
            .chain(constructors::handlers())
            .chain(constructor_standings::handlers())
            .chain(driver_standings::handlers())
            .chain(laps::handlers())
            .collect()
    }
}
