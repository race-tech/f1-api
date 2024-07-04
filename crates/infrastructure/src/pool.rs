//! Connection manager implementation for MySQL connections.
//!
//! See [`MySqlConnectionManager`].

use async_trait::async_trait;
use surrealdb::engine::remote::http::Http;
use surrealdb::opt::{Config as SurrealConfig, IntoEndpoint};
use surrealdb::Surreal;

use crate::config::DatabaseConfig;

/// An [`bb8`] connection manager for [`surreal`] connections.
#[derive(Debug)]
pub struct SurrealConnectionManager {
    config: DatabaseConfig,
    path: String,
}

#[async_trait]
impl bb8::ManageConnection for SurrealConnectionManager {
    type Connection = Surreal<<(String, SurrealConfig) as IntoEndpoint<Http>>::Client>;
    type Error = surrealdb::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(Surreal::new::<Http>((self.path.clone(), self.config.clone().into())).await?)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.health().await
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}

impl From<DatabaseConfig> for SurrealConnectionManager {
    fn from(config: DatabaseConfig) -> Self {
        let path = config.path.clone();
        SurrealConnectionManager { config, path }
    }
}
