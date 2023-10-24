// disable CMD on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::control::App;

mod control;

fn main() {
    let app = App::new();
    let _ = app.run();
}

