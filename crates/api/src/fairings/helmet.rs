use rocket::fairing::{Fairing, Info, Kind};
use rocket::{uri, Data, Orbit, Request, Rocket};

use infrastructure::ConnectionPool;

use crate::fairings::rate_limiter::SlidingWindow;
use crate::fallbacks::rocket_uri_macro_internal_ressource;

pub struct Formula1Helmet;

#[rocket::async_trait]
impl Fairing for Formula1Helmet {
    fn info(&self) -> Info {
        Info {
            name: "Formula1Helmet",
            kind: Kind::Liftoff | Kind::Request,
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

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        if req.uri().path().as_str().starts_with("/fallback") {
            req.set_uri(uri!("/fallback", internal_ressource))
        }
    }
}
