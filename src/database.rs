use anyhow::anyhow;
use deadpool_diesel::{
    mysql::{Manager, Object, Pool, Runtime},
    PoolError,
};
use diesel::MysqlConnection;
use std::env;

#[derive(Clone)]
pub struct Database(Pool);

impl Database {
    pub fn new() -> Self {
        Database(Self::create_db_pool())
    }

    fn create_db_pool() -> Pool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = Manager::new(database_url, Runtime::Tokio1);
        Pool::builder(manager)
            .max_size(8)
            .build()
            .expect("Could not build database connection pool.")
    }

    async fn get_connection(&self) -> Result<Object, PoolError> {
        self.0.get().await
    }

    pub async fn query_database<F, R>(&self, query: F) -> Result<R, anyhow::Error>
    where
        F: FnOnce(&mut MysqlConnection) -> R + Send + 'static,
        R: Send + 'static,
    {
        let conn = self.get_connection().await.map_err(|e| anyhow!(e))?;

        let result = conn
            .interact(query)
            .await
            .map_err(|e| anyhow!("Database interaction error: {:?}", e))
            .map_err(|e| anyhow!(e))?;

        Ok(result)
    }
}
