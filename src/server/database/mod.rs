use sqlx::{
    any::{install_default_drivers, AnyPoolOptions},
    migrate::MigrateError,
    Any, Error, Pool,
};
use std::env::var;

pub async fn init_db_pool() -> Result<Pool<Any>, Error> {
    install_default_drivers();
    let database_url = var("DATABASE_URL").expect("Database url env var missing!");
    AnyPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn run_db_migrations(db_pool: &Pool<Any>) -> Result<(), MigrateError> {
    sqlx::migrate!("src/server/database/migrations")
        .run(db_pool)
        .await
}
