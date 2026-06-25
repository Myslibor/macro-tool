// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod keyboard_handler;
mod macro_s;

fn main() {
    macro_tools_lib::run()
}
