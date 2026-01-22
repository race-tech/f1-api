use sea_orm::sqlx::MySqlPool;

use error::error;

pub mod config;

pub async fn create_database_conn_pool(config: &config::Config) -> Result<MySqlPool, error::Error> {
    let pool = MySqlPool::connect_with((&config.database).into())
        .await
        .map_err(|_| error!(ConnectionPool => "Failed to create connection pool"))?;
    Ok(pool)
}
