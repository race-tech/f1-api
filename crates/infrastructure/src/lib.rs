use shared::error;

pub mod config;
mod pool;

pub type Pool = bb8::Pool<pool::SurrealConnectionManager>;

#[derive(Clone)]
pub struct ConnectionPool {
    pub pool: Pool,
}

impl ConnectionPool {
    pub async fn try_from(config: config::Config) -> Result<Self, shared::error::Error> {
        let manager = pool::SurrealConnectionManager::from(config.database);
        let pool = bb8::Pool::builder()
            .max_size(20)
            .build(manager)
            .await
            .map_err(|_| error!(ConnectionPool => "Failed to create connection pool"))?;

        Ok(Self { pool })
    }
}
