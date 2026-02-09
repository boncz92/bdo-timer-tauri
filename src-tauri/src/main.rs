// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, CheckMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, AppHandle,
};
use tauri_plugin_store::StoreExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let handle = app.handle();
            
            // Setup tray
            setup_tray(handle)?;
            
            // Load saved settings
            let window = app.get_webview_window("main").unwrap();
            let store = app.store("settings.json")?;
            
            // Load always on top setting
            if let Some(always_on_top) = store.get("alwaysOnTop") {
                if let Some(value) = always_on_top.as_bool() {
                    window.set_always_on_top(value).ok();
                }
            }
            
            // Load window position and size
            if let Some(bounds) = store.get("windowBounds") {
                if let Some(obj) = bounds.as_object() {
                    let x = obj.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let y = obj.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let width = obj.get("width").and_then(|v| v.as_f64()).unwrap_or(520.0);
                    let height = obj.get("height").and_then(|v| v.as_f64()).unwrap_or(209.0);
                    
                    window.set_position(tauri::PhysicalPosition { x: x as i32, y: y as i32 }).ok();
                    window.set_size(tauri::PhysicalSize { width: width as u32, height: height as u32 }).ok();
                }
            }
            
            // Save position on move
            let handle_clone = handle.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Moved(position) = event {
                    if let Ok(store) = handle_clone.store("settings.json") {
                        if let Ok(size) = handle_clone.get_webview_window("main").unwrap().outer_size() {
                            let bounds = serde_json::json!({
                                "x": position.x,
                                "y": position.y,
                                "width": size.width,
                                "height": size.height
                            });
                            store.set("windowBounds", bounds);
                            store.save().ok();
                        }
                    }
                }
            });
            
            // Save size on resize
            let handle_clone = handle.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Resized(size) = event {
                    if let Ok(store) = handle_clone.store("settings.json") {
                        if let Ok(position) = handle_clone.get_webview_window("main").unwrap().outer_position() {
                            let bounds = serde_json::json!({
                                "x": position.x,
                                "y": position.y,
                                "width": size.width,
                                "height": size.height
                            });
                            store.set("windowBounds", bounds);
                            store.save().ok();
                        }
                    }
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.store("settings.json")?;
    let always_on_top = store.get("alwaysOnTop")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // Create menu items
    let always_on_top_item = CheckMenuItem::with_id(
        app,
        "always_on_top",
        "Always on Top",
        true,
        always_on_top,
        None::<&str>,
    )?;

    let size_small = MenuItem::with_id(app, "size_small", "Size: Small", true, None::<&str>)?;
    let size_medium = MenuItem::with_id(app, "size_medium", "Size: Medium", true, None::<&str>)?;
    let size_large = MenuItem::with_id(app, "size_large", "Size: Large", true, None::<&str>)?;

    let show_hide = MenuItem::with_id(app, "show_hide", "Show/Hide", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &always_on_top_item,
            &PredefinedMenuItem::separator(app)?,
            &size_small,
            &size_medium,
            &size_large,
            &PredefinedMenuItem::separator(app)?,
            &show_hide,
            &PredefinedMenuItem::separator(app)?,
            &quit,
        ],
    )?;
    
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("BDO Boss Timer")
        .on_menu_event(move |app, event| {
            match event.id.as_ref() {
                "always_on_top" => {
                    let window = app.get_webview_window("main").unwrap();
                    let store = app.store("settings.json").unwrap();

                    let new_state = !store.get("alwaysOnTop")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    store.set("alwaysOnTop", serde_json::Value::Bool(new_state));
                    store.save().ok();
                    window.set_always_on_top(new_state).ok();
                }
                "size_small" => {
                    let window = app.get_webview_window("main").unwrap();
                    window.set_size(tauri::PhysicalSize { width: 316, height: 96 }).ok();
                }
                "size_medium" => {
                    let window = app.get_webview_window("main").unwrap();
                    window.set_size(tauri::PhysicalSize { width: 474, height: 144 }).ok();
                }
                "size_large" => {
                    let window = app.get_webview_window("main").unwrap();
                    window.set_size(tauri::PhysicalSize { width: 632, height: 192 }).ok();
                }
                "show_hide" => {
                    let window = app.get_webview_window("main").unwrap();
                    if window.is_visible().unwrap_or(false) {
                        window.hide().ok();
                    } else {
                        window.show().ok();
                        window.set_focus().ok();
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        window.hide().ok();
                    } else {
                        window.show().ok();
                        window.set_focus().ok();
                    }
                }
            }
        })
        .build(app)?;
    
    Ok(())
}
