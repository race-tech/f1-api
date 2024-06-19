use application::graphql::Query;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::schema;

pub struct Test<'a> {
    queri: &'a str,
    expected: serde_json::Value,
    field: Option<&'a str>,
}

impl<'a> Test<'a> {
    pub fn new(queri: &'a str, expected: serde_json::Value) -> Self {
        Self {
            queri,
            expected,
            field: None,
        }
    }

    pub fn specify_field(mut self, field: &'a str) -> Self {
        self.field = Some(field);
        self
    }

    pub async fn test_ok(self) {
        let schema = setup();

        let resp = schema.execute(self.queri).await;
        assert!(resp.is_ok());
        let data = resp.data.into_json().expect("invalid json data");

        if let Some(field) = self.field {
            if let serde_json::Value::Object(o) = data {
                let sub_data = o.get(field);
                assert!(sub_data.is_some(), "no data found inside the object");
                assert!(sub_data.unwrap().is_object());

                assert_eq!(sub_data.unwrap(), &self.expected);
            } else {
                assert_eq!(data, self.expected);
            }
        } else {
            assert_eq!(data, self.expected);
        }
    }
}

pub fn setup() -> Schema<Query, EmptyMutation, EmptySubscription> {
    let config = infrastructure::config::Config::try_new().expect("valid config file");
    schema(&config).expect("couldn't load schema")
}
