use std::{collections::HashSet, default, ptr::null, sync::{Arc, Mutex}, thread::{self, JoinHandle}};

use rdev::{Event, EventType, listen, simulate};

use crate::{keyboard_handler::{js_code_to_rdev, rdev_to_js_code}, macro_s::Macro};

pub struct ListenerState {
    pub running_macros: Mutex<Vec<RunningMacro>>,
    pub main_handle: Option<JoinHandle<()>>,
    pub stop_all_flag: Arc<Mutex<bool>>,
}

pub struct RunningMacro {
    pub macro_t: Macro,
    pub stop_flag: Arc<Mutex<bool>>,
    pub handle: JoinHandle<()>,
}

impl ListenerState {
    pub fn new() -> Self {
        ListenerState {
            running_macros: Mutex::new(Vec::new()),
            main_handle: None,
            stop_all_flag: Arc::new(Mutex::new(false)),
        }
    }
}

fn spawn_key_listener(macros_this: Arc<Mutex<Vec<Macro>>>, listener_state: Arc<ListenerState>)
 -> JoinHandle<()>{
    let handle = thread::spawn(move || {
        let pressed_keys: Mutex<HashSet<String>> = Mutex::new(HashSet::new()); 

        if let Err(e) = listen(move |event| {
            let mut pressed = pressed_keys.lock().unwrap();

            match event.event_type {
                EventType::KeyPress(key) => {
                    if let Some(key_code) = rdev_to_js_code(key){
                        pressed.insert(key_code.to_string());

                        let macros = macros_this.lock().unwrap();
                        for macro_this in macros.iter(){
                            let mut matches = true;

                            for req_key in &macro_this.key_bind {
                                if !pressed.contains(req_key){
                                    matches = false;
                                    break;
                                }
                            }

                            if matches{
                                let macro_pas = macro_this.clone();
                                std::thread::spawn(move || {
                                    execute_macro(macro_pas);
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
        }){
            println!("Error: {:?}", e);
        }

    });

    return handle;
}

fn execute_macro(macro_this: Macro){
    let iterations = if macro_this.has_loop {usize::MAX} else {1};

    for _ in 0..iterations{
        for brick in &macro_this.bricks {
            if let Some(key) = js_code_to_rdev(&brick.button){

                if let Err(e) = simulate(&EventType::KeyPress(key)) {
                    eprintln!("Failed to press key: {}", e);
                }

                if let Err(e) = simulate(&EventType::KeyRelease(key)) {
                    eprintln!("Failed to release key: {}", e);
                }

                std::thread::sleep(std::time::Duration::from_secs_f64(brick.wait));

            }
        }
    }
    
}
