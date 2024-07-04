use figment::providers::{Format, Serialized, Yaml};
use figment::Figment;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Root;
use surrealdb::opt::Config as SurrealConfig;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub port: Option<u16>,
    pub database: DatabaseConfig,
    pub middlewares: Option<Vec<MiddlewareConfig>>,
}

impl Config {
    pub fn try_new() -> shared::error::Result<Self> {
        let config = Figment::from(Serialized::defaults(Config::default()))
            .merge(Yaml::file(
                std::env::var("F1_API_CONFIG").unwrap_or("config.yml".into()),
            ))
            .extract()?;
        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub path: String,
    pub user: String,
    pub pass: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MiddlewareConfig {
    Graphiql {
        #[serde(default)]
        enabled: bool,
        route: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum RateLimiterType {
    SlidingWindow,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            path: "localhost:8000".into(),
            user: "user".into(),
            pass: "password".into(),
        }
    }
}

impl From<DatabaseConfig> for SurrealConfig {
    fn from(value: DatabaseConfig) -> Self {
        SurrealConfig::new().user(Root {
            username: &value.user,
            password: &value.pass,
        })
    }
}
