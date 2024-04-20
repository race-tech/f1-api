use std::net::IpAddr;

use figment::{
    providers::{Format, Yaml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub middlewares: Option<Vec<MiddlewareConfig>>,
}

impl Config {
    pub fn try_new() -> shared::error::Result<Self> {
        let config = Figment::new().merge(Yaml::file("config.yml")).extract()?;
        Ok(config)
    }
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub name: String,
    pub ip: IpAddr,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct CacheConfig {
    pub ip: IpAddr,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MiddlewareConfig {
    RateLimiter {
        enabled: bool,
        #[serde(rename = "type")]
        ty: Option<RateLimiterType>,
        seconds: i64,
        requests: usize,
    },
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum RateLimiterType {
    SlidingWindow,
}
