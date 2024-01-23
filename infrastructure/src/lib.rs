use diesel::mysql::MysqlConnection;
use diesel::r2d2;
use std::env;
use std::ops::{Deref, DerefMut};

use shared::parameters::Series;

pub struct ConnectionPool {
    f1db_pool: DBPool,
}

impl ConnectionPool {
    pub fn try_new() -> Result<Self, ()> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        println!("Connecting to {}", database_url);

        let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .max_size(20)
            .build(manager)
            .expect("Failed to create pool.");

        Ok(Self { f1db_pool: DBPool(pool) } )
    }

    pub fn from_series(&self, series: Series) -> &DBPool {
        match series {
            Series::F1 => &self.f1db_pool,
            Series::F2 => unimplemented!(),
        }
    }
}


type InnerDBPool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;
pub struct DBPool(InnerDBPool);

impl Deref for DBPool {
    type Target = InnerDBPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DBPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
