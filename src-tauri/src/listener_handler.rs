use std::{collections::HashSet, sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Duration};

use enigo::{Direction::{Click}, Enigo, Keyboard, Settings};
use rdev::{ EventType, listen};

use crate::{AppState, keyboard_handler::{js_code_to_rdev, rdev_to_enigo_key, rdev_to_js_code}, macro_s::Macro};

pub struct ListenerState {
    pub running_macros: Mutex<Vec<RunningMacro>>,
    pub stop_all_flag: Arc<Mutex<bool>>,
    pub enigo: Mutex<Enigo>,
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
            stop_all_flag: Arc::new(Mutex::new(false)),
            enigo: Mutex::new(Enigo::new(&Settings::default()).unwrap()),
        }
    }
}

pub fn spawn_key_listener(app_state: Arc<Mutex<AppState>>, listener_state: Arc<ListenerState>)
 -> JoinHandle<()>{

    let _handle2 = spawn_cleanup_thread(listener_state.clone());

    let handle = thread::spawn(move || {
        let pressed_keys: Mutex<HashSet<String>> = Mutex::new(HashSet::new()); 

        if let Err(e) = listen(move |event| {
            let mut pressed = pressed_keys.lock().unwrap();

            if *listener_state.stop_all_flag.lock().unwrap(){
                //println!("Listener is stopped");
                return;
            }

            match event.event_type {
                EventType::KeyPress(key) => {
                    if let Some(key_code) = rdev_to_js_code(key){
                        let pres_check = pressed.insert(key_code.to_string());
                        if pres_check{
                            println!("Pressed {}", key_code);
                            check_macro_activation(&app_state, &listener_state, &pressed);
                        } 
                    }
                }
                EventType::KeyRelease(key) => {
                    if let Some(key_code) = rdev_to_js_code(key) {
                        pressed.remove(key_code);
                        println!("Unpressed {}", key_code);
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

pub fn spawn_cleanup_thread(listener_state: Arc<ListenerState>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(100));
            if *listener_state.stop_all_flag.lock().unwrap() {
                continue;
            }
            let mut running_macros = listener_state.running_macros.lock().unwrap();

            running_macros.retain(|mac| !(*mac.stop_flag.lock().unwrap()));
        }
    })
}

pub fn check_macro_activation(
    app_state: &Arc<Mutex<AppState>>,
    listener_state: &Arc<ListenerState>,
    pressed: &HashSet<String>)
{
    if *listener_state.stop_all_flag.lock().unwrap() {
        return;
    }

    let app_state_guard = app_state.lock().unwrap();
    let macros_guard = &app_state_guard.macros;
    let listener_state_clone = listener_state.clone();

    let mut to_stop: Vec<RunningMacro> = Vec::new();
    let mut to_start: Vec<Macro> = Vec::new();

    let mut running_macros = listener_state.running_macros.lock().unwrap();
    for macro_one in macros_guard.iter() {
        if !macro_one.key_bind.iter().all(|key| pressed.contains(key)) {
            continue;
        }

        let already_running = running_macros.iter().position(|rm| {
            rm.macro_t.name == macro_one.name && rm.macro_t.key_bind == macro_one.key_bind
        });

        if let Some(index) = already_running {
            if pressed.len() == macro_one.key_bind.len() && macro_one.key_bind.iter().all(|key| pressed.contains(key)) {
                to_stop.push(running_macros.remove(index));
            }
        } else {
            to_start.push(macro_one.clone());
        }
    }
    drop(running_macros);

    for running_macro in to_stop {
        println!("Stopping running macro: {}", running_macro.macro_t.name);
        *running_macro.stop_flag.lock().unwrap() = true;
    }

    for macro_one in to_start {
        println!("Started a macro : {}", macro_one.name);
        start_macro(macro_one, listener_state_clone.clone());
    }
}

pub fn start_macro(macro_this: Macro, listener_state: Arc<ListenerState>) {
    let stop_flag = Arc::new(Mutex::new(false));
    let stop_all_flag = listener_state.stop_all_flag.clone();

    let handle = thread::spawn({
        let macro_this = macro_this.clone();
        let stop_flag = stop_flag.clone();
        let stop_all_flag = stop_all_flag.clone();
        let listener_state_clone = listener_state.clone();
        move || {
            let mut enigo_guard = listener_state_clone.enigo.lock().unwrap();
            execute_macro(&macro_this, &stop_flag, &stop_all_flag, &mut enigo_guard);
        }
    });


    listener_state.running_macros.lock().unwrap().push(RunningMacro {
        macro_t: macro_this,
        stop_flag: stop_flag,
        handle: handle,
    });
    println!("Exiting macro starter");
}

pub fn execute_macro(macro_this: &Macro, stop_flag: &Arc<Mutex<bool>>, stop_all_flag: &Arc<Mutex<bool>>, enigo: &mut Enigo){
    let iterations = if macro_this.has_loop {usize::MAX} else {1};

    for _ in 0..iterations{
        if *stop_flag.lock().unwrap() || *stop_all_flag.lock().unwrap(){
            break;
        }

        for brick in &macro_this.bricks {
            if *stop_flag.lock().unwrap() || *stop_all_flag.lock().unwrap(){
                return;
            }

            if let Some(key) = js_code_to_rdev(&brick.button){

                let enigo_key = rdev_to_enigo_key(key).unwrap();
                let _ = enigo.key(enigo_key, Click);
                std::thread::sleep(Duration::from_secs_f64(brick.wait));

            }
        }
    }
    *stop_flag.lock().unwrap() = true;
    println!("Ending execution");
}

pub fn stop_all_macros(listener_state: &Arc<ListenerState>){
    let mut running_macros = listener_state.running_macros.lock().unwrap();
    for running_macro in running_macros.iter() {
        *running_macro.stop_flag.lock().unwrap() = true;
    }
    

    let handles: Vec<JoinHandle<()>>;
    handles = running_macros.drain(..).map(|rm| rm.handle).collect();

    for handle in handles {
        println!("stopped macro {:?}", handle.thread().id());
        let _ = handle.join();
    }

    println!("All macros stopped");
}

pub fn stop_everything(listener_state: &Arc<ListenerState>){
    *listener_state.stop_all_flag.lock().unwrap() = true;
    stop_all_macros(&listener_state);
}
