#![allow(clippy::too_many_arguments)]

mod circuit;
mod constructor;
mod constructor_standing;
mod driver;
mod driver_standing;
mod laps;

pub mod handlers {
    use crate::*;
    use rocket::Route;

    pub fn handlers() -> Vec<Route> {
        circuit::handlers()
            .into_iter()
            .chain(driver::handlers())
            .chain(constructor::handlers())
            .chain(constructor_standing::handlers())
            .chain(driver_standing::handlers())
            .chain(laps::handlers())
            .collect()
    }
}
