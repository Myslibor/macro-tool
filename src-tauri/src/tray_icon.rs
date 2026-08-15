
use tauri::{Manager, menu::{Menu, MenuItem}, tray::{ TrayIcon, TrayIconBuilder}};


pub fn create_tray_icon(app: &tauri::AppHandle) -> Option<TrayIcon>{
    let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>).ok()?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).ok()?;
    let menu = Menu::with_items(app, &[&show_item, &quit_item]).ok()?;

    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                 if let Some(window) = app.get_webview_window("main") {
                    let _ = window.destroy();
                }
                app.exit(0);
            }
            _ => {}
        })
        .build(app).ok()?;

    return Some(tray);
}