use rocket::local::blocking::Client;

use api_lib::rocket_builder;

pub fn setup() -> Client {
    assert!(dotenvy::dotenv().is_ok());

    Client::tracked(rocket_builder()).expect("invalid rocket instance")
}
