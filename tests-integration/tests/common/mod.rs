use rocket::local::blocking::Client;

use api_lib::rocket_builder;

pub fn setup() -> Client {
    Client::tracked(rocket_builder()).expect("invalid rocket instance")
}
