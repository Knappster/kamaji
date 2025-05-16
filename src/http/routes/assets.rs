use std::sync::Arc;
use tower_http::{
    services::{ServeDir, ServeFile},
    set_status::SetStatus,
};

use crate::config::Config;

pub fn service(config: Arc<Config>) -> ServeDir<SetStatus<ServeFile>> {
    ServeDir::new(config.assets_path.clone())
        .not_found_service(ServeFile::new(config.index_path.clone()))
}
