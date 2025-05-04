pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_config_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_config_table::Migration)]
    }

    fn migration_table_name() -> sea_orm::DynIden {
        Alias::new("migrations").into_iden()
    }
}
