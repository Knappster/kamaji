use sqlx::MySqlPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: MySqlPool,
}
