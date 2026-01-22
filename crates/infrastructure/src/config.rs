use figment::providers::{Format, Serialized, Yaml};
use figment::Figment;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub port: Option<u16>,
    pub database: DatabaseConfig,
    pub middlewares: Option<Vec<MiddlewareConfig>>,
}

impl Config {
    pub fn try_new() -> error::Result<Self> {
        let config = Figment::from(Serialized::defaults(Config::default()))
            .merge(Yaml::file(
                std::env::var("F1_API_CONFIG").unwrap_or("config.yml".into()),
            ))
            .extract()?;
        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfig {
    pub name: String,
    pub hostname: String,
    pub port: u16,
    pub user: String,
    pub password: String,
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
            name: "f1db".into(),
            hostname: "127.0.0.1".into(),
            port: 3306,
            user: "user".into(),
            password: "password".into(),
        }
    }
}

impl From<&DatabaseConfig> for sea_orm::sqlx::mysql::MySqlConnectOptions {
    fn from(value: &DatabaseConfig) -> Self {
        Self::new()
            .host(&value.hostname)
            .port(value.port)
            .database(&value.name)
            .username(&value.user)
            .password(&value.password)
    }
}
