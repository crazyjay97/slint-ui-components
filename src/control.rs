use std::sync::{Arc, Mutex};


include_modules!();


impl App {
    pub fn new() -> App {
        App {
            app: Arc::new(Mutex::new(MainWindow::new().unwrap())),
        }
    }
}