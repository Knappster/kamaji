use deadpool_diesel::{
    mysql::{Manager, Object, Pool, Runtime},
    PoolError,
};
use diesel::MysqlConnection;
use std::env;

#[derive(Clone)]
pub struct Database {
    db_pool: Pool,
}

impl Database {
    pub fn new() -> Self {
        Database {
            db_pool: Self::create_db_pool(),
        }
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
        let pool = self.db_pool.get().await?;
        Ok(pool)
    }
    pub async fn query<F, R>(&self, query: F) -> R
    where
        F: FnOnce(&mut MysqlConnection) -> R + Send + 'static,
        R: Send + 'static,
    {
        let conn = self.get_connection().await.unwrap();
        conn.interact(query).await.unwrap()
    }
}
