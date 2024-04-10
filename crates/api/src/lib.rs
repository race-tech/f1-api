#![allow(clippy::too_many_arguments)]

pub mod circuit;
pub mod constructors;
pub mod drivers;
pub mod seasons;
pub mod standings;

pub mod handlers {
    use crate::*;
    use rocket::Route;

    pub fn handlers() -> Vec<Route> {
        drivers::handlers()
            .into_iter()
            .chain(constructors::handlers())
            .chain(standings::handlers())
            .chain(seasons::handlers())
            .chain(circuit::handlers())
            .collect()
    }
}
