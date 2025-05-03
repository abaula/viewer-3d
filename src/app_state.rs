pub struct AppState {
    pub visible: bool,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            visible: false,
        }
    }
}