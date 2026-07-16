use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use rdev::{EventType, listen, simulate};

use crate::keyboard_handler::rdev_to_js_code;
use crate::macro_s::Macro;

/// Spawns a thread that listens for global keybinds and executes macros
fn spawn_key_listener(
    macros: Arc<Mutex<Vec<Macro>>>,
    active: Arc<Mutex<bool>>,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let pressed_keys: Mutex<HashSet<String>> = Mutex::new(HashSet::new());

        if let Err(e) = listen(move |event| {
            let mut pressed = pressed_keys.lock().unwrap();

            match event.event_type {
                EventType::KeyPress(key) => {
                    if let Some(key_code) = rdev_to_js_code(key) {
                        pressed.insert(key_code.to_string());

                        // Check all macros for matching keybind
                        let macros = macros.lock().unwrap();
                        for macro in macros.iter() {
                            let mut matches = true;
                            for required_key in &macro.key_bind {
                                if !pressed.contains(required_key) {
                                    matches = false;
                                    break;
                                }
                            }
                            if matches && !macro.key_bind.is_empty() {
                                // Clone macro and macros Arc for the new thread
                                let macro = macro.clone();
                                let active = active.clone();
                                std::thread::spawn(move || {
                                    execute_macro(macro, active);
                                });
                            }
                        }
                    }
                }
                EventType::KeyRelease(key) => {
                    if let Some(key_code) = rdev_to_js_code(key) {
                        pressed.remove(key_code);
                    }
                }
                _ => {}
            }
            None
        }) {
            eprintln!("Error listening to keys: {}", e);
        }
    })
}

/// Executes a macro by simulating its bricks
fn execute_macro(macro_this: Macro, active: Arc<Mutex<bool>>) {
    // Check if we should still be active
    if !*active.lock().unwrap() {
        return;
    }

    // If has_loop, we'll loop forever (until stopped)
    let iterations = if macro.has_loop { usize::MAX } else { 1 };

    for _ in 0..iterations {
        // Check active flag each iteration
        if !*active.lock().unwrap() {
            break;
        }

        for brick in &macro.bricks {
            if let Some(key) = js_code_to_rdev(&brick.button) {
                // Press the key
                if let Err(e) = simulate(&EventType::KeyPress(key)) {
                    eprintln!("Failed to press key: {}", e);
                }

                // Wait
                std::thread::sleep(std::time::Duration::from_secs_f64(brick.wait));

                // Release the key
                if let Err(e) = simulate(&EventType::KeyRelease(key)) {
                    eprintln!("Failed to release key: {}", e);
                }

                // Small delay between bricks
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }

        // If looping, add a small delay before repeating
        if macro.has_loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}