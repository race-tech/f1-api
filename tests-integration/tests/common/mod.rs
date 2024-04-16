use rocket::http::uri::Origin;
use rocket::http::Header;
use rocket::local::blocking::{Client, LocalResponse};

use api_lib::rocket_builder;

pub fn setup() -> Client {
    Client::tracked(rocket_builder()).expect("invalid rocket instance")
}

pub fn get<'c, U>(client: &'c Client, uri: U) -> LocalResponse<'c>
where
    U: TryInto<Origin<'c>> + std::fmt::Display,
{
    let mut req = client.get(uri);
    req.add_header(Header::new("x-real-ip", "127.0.0.1"));
    req.dispatch()
}
