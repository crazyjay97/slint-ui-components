use std::sync::{Arc, Mutex};
use native_dialog::FileDialog;
use slint::include_modules;


include_modules!();


pub struct App {
    pub app: Arc<Mutex<MainWindow>>,
}

impl App {
    pub fn new() -> App {
        App {
            app: Arc::new(Mutex::new(MainWindow::new().unwrap())),
        }
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.init();
        let app = self.app.lock().unwrap();
        app.run()
    }

    fn init(&self){
        self.on_file_pick();
    }

    fn on_file_pick(&self) {
        let app = self.app.lock().unwrap();
        app.global::<Logic>()
            .on_file_pick(move || match App::file_pick() {
                Ok(path) => path.into(),
                Err(_) => "".into(),
            });
    }

    fn file_pick() -> Result<String, ()> {
        let path = FileDialog::new().show_open_single_file().unwrap();
        match path {
            Some(path) => {
                let s = path.to_str().unwrap();
                Ok(s.into())
            }
            None => Err(()),
        }
    }
}