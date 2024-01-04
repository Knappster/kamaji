use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub database: MySqlPool,
}
