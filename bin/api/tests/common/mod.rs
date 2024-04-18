use std::marker::PhantomData;

use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::local::blocking::{Client, LocalResponse};

use api_lib::rocket_builder_no_fairings;
use shared::prelude::{Pagination, Response, Series};

pub struct Test<'a, T, U> {
    uri: &'a str,
    series: Series,
    pagination: Option<Pagination>,
    expected: T,
    target: PhantomData<U>,
}

impl<'a, T, U> Test<'a, T, U>
where
    U: rocket::serde::DeserializeOwned + std::marker::Send + std::fmt::Debug + 'static,
    T: PartialEq<U> + std::fmt::Debug,
{
    pub fn new(uri: &'a str, series: Series, expected: T) -> Self {
        Self {
            uri,
            series,
            pagination: None,
            expected,
            target: PhantomData,
        }
    }

    pub fn pagination(mut self, pagination: Option<Pagination>) -> Self {
        self.pagination = pagination;
        self
    }

    pub fn test_ok(self) {
        let client = setup();

        let resp = get(&client, self.uri);
        assert_eq!(resp.status(), Status::Ok);
        let json = resp.into_json::<Response<U>>().unwrap();

        assert_eq!(json.series, self.series);
        assert_eq!(json.pagination, self.pagination);
        assert_eq!(self.expected, json.data);
    }
}

pub fn setup() -> Client {
    Client::tracked(rocket_builder_no_fairings()).expect("invalid rocket instance")
}

pub fn get<'c, U>(client: &'c Client, uri: U) -> LocalResponse<'c>
where
    U: TryInto<Origin<'c>> + std::fmt::Display,
{
    client.get(uri).dispatch()
}

pub fn parse_date(date: &str) -> chrono::NaiveDate {
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}

pub fn parse_time(time: &str) -> chrono::NaiveTime {
    chrono::NaiveTime::parse_from_str(time, "%H:%M:%S").unwrap()
}
