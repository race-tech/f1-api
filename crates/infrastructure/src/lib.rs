use shared::error;

pub mod config;
mod pool;

pub type Pool = r2d2::Pool<pool::MySqlConnectionManager>;
pub type Connection = r2d2::PooledConnection<pool::MySqlConnectionManager>;

#[derive(Clone)]
pub struct ConnectionPool {
    pub pool: Pool,
}

impl TryFrom<&config::Config> for ConnectionPool {
    type Error = shared::error::Error;

    fn try_from(config: &config::Config) -> Result<Self, Self::Error> {
        let manager = pool::MySqlConnectionManager::try_from(&config.database)?;
        let pool = r2d2::Pool::builder()
            .max_size(20)
            .build(manager)
            .map_err(|_| error!(ConnectionPool => "Failed to create connection pool"))?;

        Ok(Self { pool })
    }
}
