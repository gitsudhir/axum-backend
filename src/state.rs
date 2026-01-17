use crate::config::env::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
}