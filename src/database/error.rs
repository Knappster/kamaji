use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("sea_orm::{0}")]
    SeaORM(#[from] sea_orm::error::DbErr),
}
