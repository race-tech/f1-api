#![allow(clippy::too_many_arguments)]

pub mod constructors;
pub mod drivers;
mod error;

pub mod handlers {
    use crate::{constructors, drivers};
    use rocket::Route;

    pub fn handlers() -> Vec<Route> {
        drivers::handlers()
            .into_iter()
            .chain(constructors::handlers())
            .collect()
    }
}
