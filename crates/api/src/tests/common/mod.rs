use application::graphql::Query;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::schema;

pub struct Test<'a> {
    queri: &'a str,
    expected: serde_json::Value,
}

impl<'a> Test<'a> {
    pub fn new(queri: &'a str, expected: serde_json::Value) -> Self {
        Self { queri, expected }
    }

    pub async fn test_ok(self) {
        let schema = setup();

        let resp = schema.execute(self.queri).await;
        assert!(resp.is_ok());

        let data = resp.data.into_json().expect("invalid json data");

        assert_eq!(data, self.expected);
    }
}

pub fn setup() -> Schema<Query, EmptyMutation, EmptySubscription> {
    let config = infrastructure::config::Config::try_new().expect("valid config file");
    schema(&config).expect("couldn't load schema")
}
