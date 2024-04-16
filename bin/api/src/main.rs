#[macro_use]
extern crate rocket;

use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    api_lib::rocket_builder()
}
