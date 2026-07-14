use std::sync::Arc;
use crate::config::AppConfig;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) config: Arc<AppConfig>,
}
