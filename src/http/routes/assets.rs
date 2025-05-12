use crate::config::Config;
use tower_http::{
    services::{ServeDir, ServeFile},
    set_status::SetStatus,
};

pub fn service(config: Config) -> ServeDir<SetStatus<ServeFile>> {
    ServeDir::new(config.assets_path).not_found_service(ServeFile::new(config.index_path))
}
