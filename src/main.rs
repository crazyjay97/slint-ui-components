// disable CMD on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod control;

fn main() {
    println!("Hello, world!");
}
