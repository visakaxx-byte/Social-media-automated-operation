mod schema;
mod operations;

pub use operations::{Database, DbOperations};
pub use schema::create_tables as init_database;

use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

pub fn initialize_database<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let conn = Connection::open(path)?;
    schema::create_tables(&conn)?;
    Ok(conn)
}
