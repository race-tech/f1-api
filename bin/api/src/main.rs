#[macro_use]
extern crate rocket;

use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/api", api_lib::handlers::handlers())
        .manage(infrastructure::ConnectionPool::try_new().unwrap())
}
