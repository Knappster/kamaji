use sqlx::mysql::MySqlPoolOptions;
use sqlx::{migrate::MigrateError, Error, MySqlPool};
use std::env::var;
use std::process::exit;
use tracing::{debug, instrument};

#[instrument]
pub async fn init_db_pool() -> Result<MySqlPool, Error> {
    let database_url = var("DATABASE_URL").unwrap_or_else(|e| {
        tracing::error!("DATABASE_URL: {}", e);
        exit(1)
    });

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
