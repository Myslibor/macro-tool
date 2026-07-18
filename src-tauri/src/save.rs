use std::path::Path;

use crate::AppState;

pub fn save_state_to_json(data: &AppState) {
    let json = serde_json::to_string_pretty(data).unwrap();
    std::fs::write("macros_save.json", json).unwrap();

    println!("saved to json succesfuly");
}

pub fn load_json_to_state() -> Option<AppState> {
    if Path::new("macros_save.json").exists() {
        let state = std::fs::read_to_string("macros_save.json").unwrap();

        println!("loaded the json succesfuly");
        return serde_json::from_str(&state).unwrap();
    } else {
        return None;
    }
}
