use sqlx::{Any, Pool};

#[derive(Clone)]
pub struct AppState {
    pub database: Pool<Any>,
}
