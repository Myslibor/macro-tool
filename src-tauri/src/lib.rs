use std::f64;
use std::sync::{Arc, Mutex};

use rdev::Key;

use crate::keyboard_handler::{js_code_to_rdev, rdev_to_js_code};
use crate::macro_s::{Brick, Macro};

mod keyboard_handler;
mod macro_s;

struct AppState {
    macros: Vec<Macro>,
    new_macro: Macro,
    selected_key: String,
    selected_time: f64,
}

#[tauri::command]
fn read_key(key_name: String, key_code: String, state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut state = state.lock().unwrap();
    println!("{} - {}", key_name, key_code);
    state.selected_key = key_code;
}

#[tauri::command]
fn set_time( time: f64, state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut state = state.lock().unwrap();
    state.selected_time = time;
}

#[tauri::command]
fn create_new_macro(state: tauri::State<Arc<Mutex<AppState>>>){
    let mut state = state.lock().unwrap();

    let mut new_macro = Macro{
        bricks: Vec::new(),
        key_bind: format!(" "),
        has_loop: false,
    };

    state.new_macro = new_macro;

}

#[tauri::command]
fn add_brick(state: tauri::State<Arc<Mutex<AppState>>>){
    let mut state = state.lock().unwrap();

    let mut new_brick = Brick{
        button: state.selected_key.clone(),
        wait: state.selected_time,
    };

    println!("{:?}",new_brick);

    state.new_macro.bricks.push(new_brick);
}

//getters for js

#[tauri::command]
fn get_selected_key(state: tauri::State<Arc<Mutex<AppState>>>) -> String{
    let mut state = state.lock().unwrap();

    return state.selected_key.clone(); 
}

#[tauri::command]
fn get_new_macro(state: tauri::State<Arc<Mutex<AppState>>>) -> Macro{
    let state = state.lock().unwrap();
    return state.new_macro.clone();
}  

#[tauri::command]
fn get_macros(state: tauri::State<Arc<Mutex<AppState>>>) -> Vec<Macro>{
    let state = state.lock().unwrap();
    return state.macros.clone();
}

#[tauri::command]
fn get_macro(index: usize, state: tauri::State<Arc<Mutex<AppState>>>) -> Macro{
    let state = state.lock().unwrap();
    return state.macros.get(index).cloned().unwrap();
}          

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(AppState{
            macros: Vec::new(),
            new_macro: Macro{
                bricks: Vec::new(),
                key_bind: format!(" "),
                has_loop: false,
            },
            selected_key: "KeyA".into(),
            selected_time: 1.0,
        })))
        .invoke_handler(tauri::generate_handler![
            read_key,
            set_time,
            create_new_macro,
            add_brick,
            get_macro,
            get_new_macro,
            get_macros
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
