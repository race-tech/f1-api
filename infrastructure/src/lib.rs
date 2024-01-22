use diesel::mysql::MysqlConnection;
use dotenvy::dotenv;
use std::env;

use shared::parameters::Series;

pub struct Connection {
    pool: MysqlConnection,
}

impl Connection {
    pub fn pool_from_series(&mut self, series: Series) -> &mut MysqlConnection {
        match series {
            Series::F1 => &mut self.pool,
            Series::F2 => &mut self.pool,
        }
    }
}

impl Default for Connection {
    fn default() -> Self {
        use diesel::Connection;

        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        println!("Connecting to {}", database_url);

        Self {
            pool: MysqlConnection::establish(&database_url)
                .expect("Failed to connect to database."),
        }
    }
}
