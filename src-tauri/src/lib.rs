use std::f64;
use std::sync::{Arc, Mutex};

use rdev::Key;

use crate::keyboard_handler::js_code_to_rdev;
use crate::macro_s::Macro;

mod keyboard_handler;
mod macro_s;

struct AppState {
    macros: Vec<Macro>,
    selected_key: Key,
    selected_time: f64,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_key(state: tauri::State<Arc<Mutex<AppState>>>, key_name: String, key_code: String) {
    let mut state = state.lock().unwrap();
    println!("{} - {}", key_name, key_code);
    let key = js_code_to_rdev(key_code.as_str()).unwrap();
    state.selected_key = key;
}

#[tauri::command]
fn set_time(state: tauri::State<Arc<Mutex<AppState>>>, time: f64) {
    let mut state = state.lock().unwrap();
    state.selected_time = time;
}

#[tauri::command]
fn create_new_macro(state: tauri::State<Arc<Mutex<AppState>>> ){
    let mut new_macro = Macro{
        bricks: Vec::new(),
        key_bind: format!(" "),
        has_loop: false,
    };
    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(AppState{
            macros: Vec::new(),
            selected_key: Key::KeyA,
            selected_time: 1.0,
        })))
        .invoke_handler(tauri::generate_handler![greet, read_key, set_time, create_new_macro])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
