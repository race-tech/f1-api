use rocket::http::uri::Origin;
use rocket::local::blocking::{Client, LocalResponse};

use api_lib::rocket_builder_no_fairings;

pub fn setup() -> Client {
    Client::tracked(rocket_builder_no_fairings()).expect("invalid rocket instance")
}

pub fn get<'c, U>(client: &'c Client, uri: U) -> LocalResponse<'c>
where
    U: TryInto<Origin<'c>> + std::fmt::Display,
{
    client.get(uri).dispatch()
}
