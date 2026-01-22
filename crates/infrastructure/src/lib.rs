use shared::error;

pub mod config;
mod pool;
use sea_orm::sqlx::MySqlPool;

pub type Pool = r2d2::Pool<pool::MySqlConnectionManager>;
pub type Connection = r2d2::PooledConnection<pool::MySqlConnectionManager>;

pub async fn create_database_conn_pool(
    config: &config::Config,
) -> Result<MySqlPool, shared::error::Error> {
    let pool = MySqlPool::connect_with((&config.database).into())
        .await
        .map_err(|_| error!(ConnectionPool => "Failed to create connection pool"))?;
    Ok(pool)
}
