use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Orbit, Rocket};

use infrastructure::ConnectionPool;

use crate::guards::rate_limiter::SlidingWindow;

pub struct Formula1Helmet;

#[rocket::async_trait]
impl Fairing for Formula1Helmet {
    fn info(&self) -> Info {
        Info {
            name: "Formula1Helmet",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        if rocket.config().ip_header.is_none()
            || rocket.state::<ConnectionPool>().is_none()
            || rocket.state::<SlidingWindow>().is_none()
        {
            rocket.shutdown().notify()
        }
    }
}
