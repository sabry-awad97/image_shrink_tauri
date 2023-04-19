use std::sync::{Arc, Mutex};

pub enum DevToolsState {
    Closed,
    Opened,
}

pub struct AppState {
    pub dev_tools_state: Mutex<DevToolsState>,
}

impl AppState {
    fn new() -> Self {
        Self {
            dev_tools_state: Mutex::new(DevToolsState::Closed),
        }
    }
}

lazy_static! {
    pub static ref APP_STATE: Arc<AppState> = Arc::new(AppState::new());
}
