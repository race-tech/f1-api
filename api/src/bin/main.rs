#[macro_use]
extern crate rocket;

use api::handler;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![handler::drivers])
}
