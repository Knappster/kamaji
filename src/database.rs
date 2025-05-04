use migration::{Migrator, MigratorTrait};
use sea_orm::{Database as SeaORMDatabase, DatabaseConnection};
use std::env;

#[derive(Clone)]
pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Self {
        let connection = Self::create_connection().await;
        Migrator::up(&connection, None)
            .await
            .expect("Migrations failed!");

        Database { connection }
    }

    async fn create_connection() -> DatabaseConnection {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        SeaORMDatabase::connect(database_url)
            .await
            .expect("Database connection failed!")
    }

    pub async fn get_connection(&self) -> DatabaseConnection {
        self.connection.clone()
    }
}
