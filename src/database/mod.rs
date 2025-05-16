pub mod error;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database as SeaORMDatabase, DatabaseConnection};
use std::sync::Arc;

use crate::config::Config;
use error::DatabaseError;

#[derive(Debug, Clone)]
pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub async fn new(config: Arc<Config>) -> Result<Self, DatabaseError> {
        let connection = Self::create_connection(config).await?;
        Migrator::up(&connection, None).await?;

        Ok(Database { connection })
    }

    async fn create_connection(config: Arc<Config>) -> Result<DatabaseConnection, DatabaseError> {
        Ok(SeaORMDatabase::connect(config.database_url.clone()).await?)
    }

    pub async fn get_connection(&self) -> DatabaseConnection {
        self.connection.clone()
    }
}
