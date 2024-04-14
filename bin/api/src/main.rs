#[macro_use]
extern crate rocket;

use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(api_lib::fairings::rate_limiter::RateLimiter)
        .mount("/api", api_lib::handlers::handlers())
        .mount("/fallback", api_lib::fallbacks::handlers())
        .manage(infrastructure::ConnectionPool::try_new().unwrap())
        .manage(api_lib::fairings::rate_limiter::SlidingWindow::new(
            10,
            chrono::Duration::seconds(60),
        ))
}
