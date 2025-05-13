pub mod error;

use error::DatabaseError;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database as SeaORMDatabase, DatabaseConnection};

use crate::config::ConfigType;

#[derive(Debug, Clone)]
pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub async fn new(config: ConfigType) -> Result<Self, DatabaseError> {
        let connection = Self::create_connection(config).await?;
        Migrator::up(&connection, None).await?;

        Ok(Database { connection })
    }

    async fn create_connection(config: ConfigType) -> Result<DatabaseConnection, DatabaseError> {
        let config = config.lock().await.clone();
        Ok(SeaORMDatabase::connect(config.database_url).await?)
    }

    pub async fn get_connection(&self) -> DatabaseConnection {
        self.connection.clone()
    }
}
