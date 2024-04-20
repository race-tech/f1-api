use shared::error;
use shared::parameters::Series;

pub mod config;
mod pool;

pub type Pool = r2d2::Pool<pool::MySqlConnectionManager>;
pub type CachePool = r2d2::Pool<pool::RedisClient>;
pub type Connection = r2d2::PooledConnection<pool::MySqlConnectionManager>;

#[derive(Clone)]
pub struct ConnectionPool {
    f1db_pool: Pool,
    pub cache: CachePool,
}

impl TryFrom<&config::Config> for ConnectionPool {
    type Error = shared::error::Error;

    fn try_from(config: &config::Config) -> Result<Self, Self::Error> {
        let manager = pool::MySqlConnectionManager::try_from(&config.database)?;
        let f1db_pool = r2d2::Pool::builder()
            .max_size(20)
            .build(manager)
            .map_err(|_| error!(ConnectionPool => "Failed to create connection pool"))?;

        let cache = r2d2::Pool::builder()
            .max_size(10)
            .build(pool::RedisClient::try_from(&config.cache)?)
            .map_err(
                |_| error!(ConnectionPool => "Failed to create rate limiter connection pool"),
            )?;

        Ok(Self { f1db_pool, cache })
    }
}

impl ConnectionPool {
    pub fn from_series(&self, series: Series) -> &Pool {
        match series {
            Series::F1 => &self.f1db_pool,
            Series::F2 => unimplemented!(),
        }
    }
}
