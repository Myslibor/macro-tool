use std::f64;

use crate::keyboard_handler::js_code_to_rdev;

mod keyboard_handler;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_key(key_name: String, key_code: String) {
    println!("{} - {}", key_name, key_code);
    let key = js_code_to_rdev(key_code.as_str()).unwrap();
}

#[tauri::command]
fn set_time(time: f64) {
    println!("{}", time);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, read_key, set_time])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
