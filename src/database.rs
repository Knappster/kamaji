use deadpool_diesel::{
    mysql::{Manager, Object, Pool, Runtime},
    PoolError,
};
use std::env;

#[derive(Clone)]
pub struct Database(pub Pool);

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

    pub async fn get_connection(&self) -> Result<Object, PoolError> {
        match self.0.get().await {
            Ok(conn) => Ok(conn),
            Err(e) => (|e| {
                tracing::error!("{}", e);
                Err(e)
            })(e),
        }
    }
}
