use async_graphql::dynamic::Schema;

#[path = "../../src/query_root.rs"]
mod query_root;

#[allow(dead_code)]
pub struct Test<'a> {
    query: &'a str,
    expected: Option<serde_json::Value>,
    schema: Schema,
}

#[allow(dead_code)]
impl<'a> Test<'a> {
    pub async fn new(query: &'a str) -> Test<'a> {
        Test {
            query,
            expected: None,
            schema: setup().await,
        }
    }

    pub fn with_expected(mut self, expected: serde_json::Value) -> Test<'a> {
        self.expected = Some(expected);
        self
    }

    pub async fn test(self) -> Test<'a> {
        let resp = self.schema.execute(self.query).await;
        assert!(resp.is_ok(), "response is not valid");

        let data = resp.data.into_json();
        assert!(data.is_ok(), "data is not valid json");
        if let Some(ref expected) = self.expected {
            let data = data.unwrap();
            assert_eq!(&data, expected, "data is not the one expected");
        }
        self
    }

    pub async fn test_map<F>(self, f: F) -> Test<'a>
    where
        F: FnOnce(serde_json::Value),
    {
        let resp = self.schema.execute(self.query).await;
        assert!(resp.is_ok(), "response is not valid");

        let data = resp.data.into_json();
        assert!(data.is_ok(), "data is not valid json");
        let data = data.unwrap();
        f(data);
        self
    }
}

pub fn extract_nodes(res: serde_json::Value, key: &str) -> Vec<serde_json::Value> {
    if let serde_json::Value::Object(object) = res {
        if !object.contains_key(key) {
            panic!("key not found in object: {object:?}");
        }

        if let Some(inner_object) = object.get(key).unwrap().as_object() {
            inner_object
                .get("nodes")
                .expect("unable to find nodes inside inner object")
                .as_array()
                .expect("nodes is not an array")
                .clone()
        } else {
            panic!("cannot find inner object at {key} inside {object:?}");
        }
    } else {
        panic!("{res} is not an object");
    }
}

async fn setup() -> Schema {
    let config = infrastructure::config::Config::try_new().expect("unable to create a config file");
    let pool = infrastructure::create_database_conn_pool(&config)
        .await
        .expect("unable to create database pool");

    query_root::schema(pool.into(), None, None).unwrap()
}
