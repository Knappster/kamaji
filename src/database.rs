use migration::{Migrator, MigratorTrait};
use sea_orm::{Database as SeaORMDatabase, DatabaseConnection};
use std::sync::Arc;
use std::sync::Mutex;

use crate::config::Config;

#[derive(Clone)]
pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub async fn new(config: Arc<Mutex<Config>>) -> Self {
        let connection = Self::create_connection(config).await;
        Migrator::up(&connection, None)
            .await
            .expect("Migrations failed!");

        Database { connection }
    }

    async fn create_connection(config: Arc<Mutex<Config>>) -> DatabaseConnection {
        let config = config.lock().unwrap().clone();
        SeaORMDatabase::connect(config.database_url)
            .await
            .expect("Database connection failed!")
    }

    pub async fn get_connection(&self) -> DatabaseConnection {
        self.connection.clone()
    }
}
