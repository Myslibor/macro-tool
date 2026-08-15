use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::listener_handler::{spawn_key_listener, stop_all_macros, stop_everything};
use crate::macro_s::{Brick, Macro};
use crate::save::save_state_to_json;
use crate::tray_icon::create_tray_icon;

mod keyboard_handler;
mod listener_handler;
mod macro_s;
mod save;
mod tray_icon;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct AppState {
    macros: Vec<Macro>,
    new_macro: Macro,
    selected_key: String,
    selected_time: f64,
}

#[tauri::command]
fn read_key(key_code: String, state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut state = state.lock().unwrap();
    println!("{}", key_code);
    state.selected_key = key_code;
}

#[tauri::command]
fn set_time(time: f64, state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut state = state.lock().unwrap();
    state.selected_time = time;
}

#[tauri::command]
fn create_new_macro(state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut state = state.lock().unwrap();

    let new_macro = Macro::new();

    state.new_macro = new_macro;
}

#[tauri::command]
fn edit_macro(index: usize, state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut state = state.lock().unwrap();

    state.new_macro = state.macros.get(index).cloned().unwrap();
    println!("new macro is now macro nr.{}", index);

    state.macros.remove(index);
    println!("removed macro macro nr.{}", index);
}

#[tauri::command]
fn add_brick(state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut state = state.lock().unwrap();

    let new_brick = Brick {
        button: state.selected_key.clone(),
        wait: state.selected_time,
    };

    println!("{:?}", new_brick);

    state.new_macro.bricks.push(new_brick);
}

#[tauri::command]
fn save_macro(state: tauri::State<Arc<Mutex<AppState>>>) -> bool {
    let mut state = state.lock().unwrap();

    if state.new_macro.bricks.is_empty()
        || state.new_macro.key_bind.is_empty()
        || state.new_macro.name.is_empty()
    {
        return false;
    }

    let to_push = state.new_macro.clone();
    state.macros.push(to_push);
    return true;
}

#[tauri::command]
fn save_everything(app_handle: tauri::AppHandle, state: tauri::State<Arc<Mutex<AppState>>>) {
    let state = state.lock().unwrap();

    save_state_to_json(&app_handle ,&state);
}

#[tauri::command]
fn delete_brick(index: usize, state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut state = state.lock().unwrap();

    state.new_macro.bricks.remove(index);
    println!("deleted brick nr.{}", index);
}

#[tauri::command]
fn set_new_name(name: String, state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut state = state.lock().unwrap();
    state.new_macro.name = name;
}

#[tauri::command]
fn set_key_bind(key_bind: Vec<String>, state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut state = state.lock().unwrap();
    state.new_macro.key_bind = key_bind;
}

#[tauri::command]
fn set_loop(has_loop: bool, state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut state = state.lock().unwrap();
    state.new_macro.has_loop = has_loop;
    println!("new macro loop state is {}", has_loop);
}

#[tauri::command]
fn start_listener(
    state: tauri::State<Arc<Mutex<AppState>>>,
    listener_state: tauri::State<Arc<listener_handler::ListenerState>>,
    listener_handle: tauri::State<Arc<Mutex<Option<JoinHandle<()>>>>>,
) -> bool {
    let mut handle_guard = listener_handle.lock().unwrap();

    if handle_guard.is_some() {
        if *listener_state.stop_all_flag.lock().unwrap(){
            *listener_state.stop_all_flag.lock().unwrap() = false;
            println!("Listener is turned on again");
            return true;
        }
        return false;
    }

    let app_state = state.inner().clone();
    let listener_state_arc = listener_state.inner().clone();
    let handle = spawn_key_listener(app_state, listener_state_arc);

    println!("Listiner spawned");

    *handle_guard = Some(handle);
    return true;
}

#[tauri::command]
fn stop_listener(
    listener_handle: tauri::State<Arc<Mutex<Option<JoinHandle<()>>>>>,
    listener_state: tauri::State<Arc<listener_handler::ListenerState>>,
) -> bool {
    let handle_guard = listener_handle.lock().unwrap();

    if let Some(_handle) = &*handle_guard {
        stop_everything(&listener_state.inner().clone());
        println!("Listiner stoped");
        return true;
    } else {
        return false;
    }
}

#[tauri::command]
fn stop_all_macros_command(listener_state: tauri::State<Arc<listener_handler::ListenerState>>) {
    stop_all_macros(&listener_state.inner().clone());
}

#[tauri::command]
fn is_listener_running(listener_handle: tauri::State<Arc<Mutex<Option<JoinHandle<()>>>>>) -> bool {
    return listener_handle.lock().unwrap().is_some();
}

//getters for js

#[tauri::command]
fn get_selected_key(state: tauri::State<Arc<Mutex<AppState>>>) -> String {
    let state = state.lock().unwrap();

    return state.selected_key.clone();
}

#[tauri::command]
fn get_new_macro(state: tauri::State<Arc<Mutex<AppState>>>) -> Macro {
    let state = state.lock().unwrap();
    return state.new_macro.clone();
}

#[tauri::command]
fn get_macros(state: tauri::State<Arc<Mutex<AppState>>>) -> Vec<Macro> {
    let state = state.lock().unwrap();
    return state.macros.clone();
}

#[tauri::command]
fn get_macro(index: usize, state: tauri::State<Arc<Mutex<AppState>>>) -> Macro {
    let state = state.lock().unwrap();
    return state.macros.get(index).cloned().unwrap();
}

//getters for new

#[tauri::command]
fn get_new_name(state: tauri::State<Arc<Mutex<AppState>>>) -> String {
    let state = state.lock().unwrap();
    return state.new_macro.name.clone();
}

#[tauri::command]
fn get_key_bind(state: tauri::State<Arc<Mutex<AppState>>>) -> Vec<String> {
    let state = state.lock().unwrap();
    return state.new_macro.key_bind.clone();
}

#[tauri::command]
fn get_new_has_loop(state: tauri::State<Arc<Mutex<AppState>>>) -> bool {
    let state = state.lock().unwrap();
    return state.new_macro.has_loop;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let init_state = save::load_json_to_state(app.handle()).unwrap_or(AppState {
                macros: Vec::new(),
                new_macro: Macro::new(),
                selected_key: "KeyA".into(),
                selected_time: 1.0,
            });
            let _tray = create_tray_icon(&app.handle());

            let window = app.get_webview_window("main").unwrap();
            let w = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = w.hide();
                }
            });

            app.manage(Arc::new(Mutex::new(init_state)));
            app.manage(Arc::new(listener_handler::ListenerState::new()));
            app.manage(Arc::new(Mutex::new(Option::<JoinHandle<()>>::None)));
            Ok(())
        })
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
            get_key_bind,
            set_loop,
            edit_macro,
            get_new_has_loop,
            start_listener,
            stop_listener,
            stop_all_macros_command,
            is_listener_running,
            get_selected_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
