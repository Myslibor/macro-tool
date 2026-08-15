use std::path::{PathBuf};

use tauri::{AppHandle, Manager};

use crate::AppState;

fn save_file_path(app_handle: &AppHandle) -> Option<PathBuf> {
    let dir = app_handle.path().app_data_dir().ok()?;
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join("macros_save.json"))
}

pub fn save_state_to_json(app_handle: &tauri::AppHandle ,data: &AppState) {
    let Some(path_t) = save_file_path(app_handle) else { return; };
    let json = serde_json::to_string_pretty(data).unwrap();
    std::fs::write(path_t, json).unwrap();
    println!("saved to json succesfuly");
}

pub fn load_json_to_state(app_handle: &tauri::AppHandle) -> Option<AppState> {
    let path = save_file_path(app_handle)?;
    if path.exists() {
        let state = std::fs::read_to_string(path).unwrap();
        println!("loaded the json succesfuly");
        return serde_json::from_str(&state).unwrap();
    }
    None
}
