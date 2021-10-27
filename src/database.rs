use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{Connection, SqliteConnection};
use itconfig::get_env_or_default;
use lazy_static::lazy_static;
use tracing::{debug, info};

use crate::constants::*;
use crate::errors::ApiError;

embed_migrations!("./migrations");

type DbPool = Pool<ConnectionManager<SqliteConnection>>;
type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

lazy_static! {
  static ref POOL: DbPool = {
     // TODO: needs to be sanitized? (don't know yet...)
    let db_filename: String = get_env_or_default("DATABASE_FILENAME", "hltvapi.db");
    let db_url: String = format!("file:{}", db_filename);

    debug!("Creating DB Pool for '{}'..", db_url);
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool_size = match cfg!(test) {
      true => 1,
      false => 1,
    };
    r2d2::Builder::new().max_size(pool_size).build(manager).expect(CREATE_DB_POOL_ERROR)
  };
}

pub fn connection() -> Result<DbConnection, ApiError> {
  POOL
    .get()
    .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}

pub fn initialize() -> Result<(), ApiError> {
  info!("Initializing DB Pool..");
  // lazy_static::initialize(&POOL);
  let conn = connection().expect(GET_DB_CONNECTION_ERROR);
  if cfg!(test) {
    conn
      .begin_test_transaction()
      .expect("Failed to start transaction");
  }
  Ok(embedded_migrations::run(&conn).unwrap())
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_initialize() {
    assert_eq!(initialize().unwrap(), ());
  }
}
