//! Connection manager implementation for MySQL connections.
//!
//! See [`MySqlConnectionManager`].

use mysql::{error::Error as MySqlError, prelude::*, Conn, Opts, OptsBuilder};
use redis::{Client, ConnectionLike, RedisError};

/// An [`r2d2`] connection manager for [`mysql`] connections.
#[derive(Clone, Debug)]
pub struct MySqlConnectionManager {
    params: Opts,
}

impl MySqlConnectionManager {
    /// Constructs a new MySQL connection manager from `params`.
    pub fn new(params: OptsBuilder) -> MySqlConnectionManager {
        MySqlConnectionManager {
            params: Opts::from(params),
        }
    }
}

impl r2d2::ManageConnection for MySqlConnectionManager {
    type Connection = Conn;
    type Error = MySqlError;

    fn connect(&self) -> Result<Conn, Self::Error> {
        Conn::new(self.params.clone())
    }

    fn is_valid(&self, conn: &mut Conn) -> Result<(), Self::Error> {
        conn.query("SELECT version()").map(|_: Vec<String>| ())
    }

    fn has_broken(&self, conn: &mut Conn) -> bool {
        self.is_valid(conn).is_err()
    }
}

pub struct RedisClient(Client);

impl TryFrom<String> for RedisClient {
    type Error = shared::error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(redis::Client::open(value)?))
    }
}

impl r2d2::ManageConnection for RedisClient {
    type Error = RedisError;
    type Connection = redis::Connection;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.0.get_connection()
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        if conn.check_connection() {
            Ok(())
        } else {
            Err(RedisError::from(std::io::Error::from(
                std::io::ErrorKind::BrokenPipe,
            )))
        }
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        !conn.is_open()
    }
}
