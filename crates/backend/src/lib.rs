mod macros;

mod models;
mod query;
mod schema;

pub type DieselBackend = diesel::mysql::Mysql;
