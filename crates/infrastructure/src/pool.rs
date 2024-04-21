//! Connection manager implementation for MySQL connections.
//!
//! See [`MySqlConnectionManager`].

use mysql::{error::Error as MySqlError, prelude::*, Conn, Opts, OptsBuilder};
use redis::{Client, ConnectionLike, RedisError};

use crate::config::{CacheConfig, DatabaseConfig};

/// An [`r2d2`] connection manager for [`mysql`] connections.
#[derive(Clone, Debug)]
pub struct MySqlConnectionManager {
    opts: Opts,
}

impl TryFrom<&DatabaseConfig> for MySqlConnectionManager {
    type Error = shared::error::Error;

    fn try_from(
        DatabaseConfig {
            name,
            hostname,
            port,
            user,
            password,
        }: &DatabaseConfig,
    ) -> Result<Self, Self::Error> {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(hostname))
            .db_name(Some(name))
            .user(Some(user))
            .pass(Some(password))
            .tcp_port(*port)
            .into();

        Ok(Self { opts })
    }
}

impl r2d2::ManageConnection for MySqlConnectionManager {
    type Connection = Conn;
    type Error = MySqlError;

    fn connect(&self) -> Result<Conn, Self::Error> {
        Conn::new(self.opts.clone())
    }

    fn is_valid(&self, conn: &mut Conn) -> Result<(), Self::Error> {
        conn.query("SELECT version()").map(|_: Vec<String>| ())
    }

    fn has_broken(&self, conn: &mut Conn) -> bool {
        self.is_valid(conn).is_err()
    }
}

pub struct RedisClient(Client);

impl TryFrom<&CacheConfig> for RedisClient {
    type Error = shared::error::Error;

    fn try_from(CacheConfig { hostname, port }: &CacheConfig) -> Result<Self, Self::Error> {
        let url = format!("redis://{}:{}", hostname, port);
        Ok(Self(redis::Client::open(url)?))
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
