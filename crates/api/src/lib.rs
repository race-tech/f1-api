#![allow(clippy::too_many_arguments)]

use rocket::{Build, Rocket};

mod catchers;
mod circuits;
mod constructor_standings;
mod constructors;
mod driver_standings;
mod drivers;
mod fairings;
mod guards;
mod laps;
mod pit_stops;
mod races;
mod seasons;
mod status;

pub fn rocket_builder() -> Rocket<Build> {
    rocket::build()
        .attach(fairings::helmet::Formula1Helmet)
        .register("/", catchers::catchers())
        .mount("/api", handlers::handlers())
        .manage(infrastructure::ConnectionPool::try_new().unwrap())
        .manage(guards::rate_limiter::SlidingWindow::default())
}

mod handlers {
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
            .chain(pit_stops::handlers())
            .chain(races::handlers())
            .chain(seasons::handlers())
            .chain(status::handlers())
            .collect()
    }
}
