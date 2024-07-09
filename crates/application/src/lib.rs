mod graphql;
mod pagination;
mod surreal;

type QueryResult =
    shared::error::Result<(String, std::collections::HashMap<String, serde_json::Value>)>;
