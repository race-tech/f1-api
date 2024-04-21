use figment::providers::{Format, Serialized, Yaml};
use figment::Figment;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub middlewares: Option<Vec<MiddlewareConfig>>,
}

impl Config {
    pub fn try_new() -> shared::error::Result<Self> {
        let config = Figment::from(Serialized::defaults(Config::default()))
            .merge(Yaml::file(
                std::env::var("PURPLE_SECTOR_CONFIG").unwrap_or("config.yml".into()),
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
pub struct CacheConfig {
    pub hostname: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MiddlewareConfig {
    RateLimiter {
        #[serde(default)]
        enabled: bool,
        #[serde(rename = "type")]
        ty: Option<RateLimiterType>,
        seconds: i64,
        requests: usize,
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

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            hostname: "127.0.0.1".into(),
            port: 6379,
        }
    }
}
