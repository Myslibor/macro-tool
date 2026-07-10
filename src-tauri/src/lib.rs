use std::f64;
use std::sync::{Arc, Mutex};

use rdev::Key;
use serde::{Deserialize, Serialize};

use crate::keyboard_handler::{js_code_to_rdev, rdev_to_js_code};
use crate::macro_s::{Brick, Macro};
use crate::save::save_state_to_json;

mod keyboard_handler;
mod macro_s;
mod save;

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    let mut new_macro = Macro::new();

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

#[tauri::command]
fn save_macro(state: tauri::State<Arc<Mutex<AppState>>>) -> bool{
    let mut state = state.lock().unwrap();

    if state.new_macro.bricks.is_empty() || 
    state.new_macro.key_bind.is_empty() || 
    state.new_macro.name.is_empty(){
        return false;
    }


    let to_push = state.new_macro.clone();
    state.macros.push(to_push);
    return true;
}

#[tauri::command]
fn save_everything(state: tauri::State<Arc<Mutex<AppState>>>){
    let mut state = state.lock().unwrap();

    save_state_to_json(&state);
}

#[tauri::command]
fn delete_brick(index: usize, state: tauri::State<Arc<Mutex<AppState>>>){
    let mut state = state.lock().unwrap();

    state.new_macro.bricks.remove(index);
    println!("deleted brick nr.{}",index);
}

#[tauri::command]
fn set_new_name(name: String, state: tauri::State<Arc<Mutex<AppState>>>){
    let mut state = state.lock().unwrap();

    state.new_macro.name = name.clone();
    println!("new name is {}", name);
}

#[tauri::command]
fn set_key_bind(key_bind: Vec<String>, state: tauri::State<Arc<Mutex<AppState>>>){
    let mut state = state.lock().unwrap();

    state.new_macro.key_bind = key_bind.clone();
    println!("new keybind is {:?}", key_bind);
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

#[tauri::command]
fn get_new_name(state: tauri::State<Arc<Mutex<AppState>>>) -> String{
    let state = state.lock().unwrap();
    return state.new_macro.name.clone();
}  

#[tauri::command]
fn get_key_bind(state: tauri::State<Arc<Mutex<AppState>>>) -> Vec<String>{
    let state = state.lock().unwrap();
    return state.new_macro.key_bind.clone();
}  

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let init_state = save::load_json_to_state().unwrap_or(AppState{
        macros: Vec::new(),
        new_macro: Macro::new(),
        selected_key: "KeyA".into(),
        selected_time: 1.0,
    });
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(init_state)))
        .invoke_handler(tauri::generate_handler![
            read_key,
            set_time,
            create_new_macro,
            add_brick,
            get_macro,
            get_new_macro,
            get_macros,
            save_macro,
            save_everything,
            delete_brick,
            get_new_name,
            set_new_name,
            set_key_bind,
            get_key_bind
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
