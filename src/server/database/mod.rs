use sqlx::mysql::MySqlPoolOptions;
use sqlx::{migrate::MigrateError, Error, MySqlPool};
use std::env::var;
use tracing::{debug, instrument};

#[instrument]
pub async fn init_db_pool() -> Result<MySqlPool, Error> {
    let database_url = var("DATABASE_URL").expect("Database url env var missing!");

    debug!(database_url);

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn run_db_migrations(db_pool: &MySqlPool) -> Result<(), MigrateError> {
    sqlx::migrate!("src/server/database/migrations")
        .run(db_pool)
        .await
}
