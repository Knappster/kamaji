use anyhow::{Context, Result};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env::var;
use tracing::debug;

pub async fn init_db_pool() -> Result<MySqlPool> {
    let database_url =
        var("DATABASE_URL").with_context(|| "DATABASE_URL environment variable missing")?;

    debug!(database_url);

    Ok(MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?)
}

pub async fn run_db_migrations(db_pool: &MySqlPool) -> Result<()> {
    Ok(sqlx::migrate!("src/server/database/migrations")
        .run(db_pool)
        .await?)
}
