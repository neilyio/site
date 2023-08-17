use std::sync::Mutex;

pub struct AppState {
    pub count: Mutex<i32>,
}
