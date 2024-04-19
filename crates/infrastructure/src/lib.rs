use std::env;

use shared::error;
use shared::parameters::Series;

mod pool;

pub type Pool = r2d2::Pool<pool::MySqlConnectionManager>;
pub type CachePool = r2d2::Pool<pool::RedisClient>;
pub type Connection = r2d2::PooledConnection<pool::MySqlConnectionManager>;

#[derive(Clone)]
pub struct ConnectionPool {
    f1db_pool: Pool,
    pub cache: CachePool,
}

impl ConnectionPool {
    pub fn try_new() -> shared::error::Result<Self> {
        let env_vars = EnvVars::try_new()?;

        let opts = mysql::OptsBuilder::new()
            .ip_or_hostname(Some(env_vars.ip_or_hostname))
            .db_name(Some(env_vars.db_name))
            .user(Some(env_vars.user))
            .pass(Some(env_vars.password))
            .tcp_port(env_vars.port.parse().map_err(
                |_| error!(ParseInt => "failed to parse port number from env variable"),
            )?);

        let manager = pool::MySqlConnectionManager::new(opts);
        let f1db_pool = r2d2::Pool::builder()
            .max_size(20)
            .build(manager)
            .map_err(|_| error!(ConnectionPool => "Failed to create connection pool"))?;

        let rate_limiter_url = env_vars.rate_limiter_config.to_string();
        let cache = r2d2::Pool::builder()
            .max_size(10)
            .build(pool::RedisClient::try_from(rate_limiter_url)?)
            .map_err(
                |_| error!(ConnectionPool => "Failed to create rate limiter connection pool"),
            )?;

        Ok(Self { f1db_pool, cache })
    }

    pub fn from_series(&self, series: Series) -> &Pool {
        match series {
            Series::F1 => &self.f1db_pool,
            Series::F2 => unimplemented!(),
        }
    }
}

struct EnvVars {
    ip_or_hostname: String,
    db_name: String,
    user: String,
    password: String,
    port: String,

    rate_limiter_config: RedisEnvConfig,
}

impl EnvVars {
    fn try_new() -> Result<Self, error::Error> {
        Ok(Self {
            ip_or_hostname: env::var("DB_IP_OR_HOSTNAME")
                .map_err(|_| error!(MissingEnvVar => "DB_IP_OR_HOSTNAME value is missing"))?,
            db_name: env::var("DB_NAME")
                .map_err(|_| error!(MissingEnvVar => "DB_NAME value is missing"))?,
            user: env::var("DB_USER")
                .map_err(|_| error!(MissingEnvVar => "DB_USER value is missing"))?,
            password: env::var("DB_PASSWORD")
                .map_err(|_| error!(MissingEnvVar => "DB_PASSWORD value is missing"))?,
            port: env::var("DB_PORT")
                .map_err(|_| error!(MissingEnvVar => "DB_PORT value is missing"))?,
            rate_limiter_config: RedisEnvConfig::try_new()?,
        })
    }
}

struct RedisEnvConfig {
    host: String,
    port: String,
}

impl RedisEnvConfig {
    fn try_new() -> Result<Self, error::Error> {
        Ok(Self {
            host: env::var("REDIS_IP_OR_HOSTNAME")
                .map_err(|_| error!(MissingEnvVar => "REDIS_IP_OR_HOSTNAME value is missing"))?,
            port: env::var("REDIS_PORT")
                .map_err(|_| error!(MissingEnvVar => "REDIS_PORT value is missing"))?,
        })
    }
}

impl std::fmt::Display for RedisEnvConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "redis://{}:{}", self.host, self.port)
    }
}
