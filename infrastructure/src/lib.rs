use diesel::mysql::MysqlConnection;
use dotenvy::dotenv;
use std::env;

pub struct Connection {
    pool: MysqlConnection,
}

impl Connection {
    pub fn pool(&mut self) -> &mut MysqlConnection {
        &mut self.pool
    }
}

impl Default for Connection {
    fn default() -> Self {
        use diesel::Connection;

        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        println!("Connecting to {}", database_url);

        Self {
            pool: MysqlConnection::establish(&database_url).expect("Failed to connect to database."),
        }
    }
}