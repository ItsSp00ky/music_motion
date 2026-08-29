pub mod audio;
pub mod config;
pub mod tray;
pub mod window;

use audio::AudioEngine;
use config::{AppConfig, ConfigManager};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use window::WindowHelper;

struct AppState {
    audio_engine: Mutex<AudioEngine>,
}

#[tauri::command]
fn get_config() -> AppConfig {
    ConfigManager::load()
}

#[tauri::command]
fn save_config(app_handle: AppHandle, config: AppConfig) {
    ConfigManager::save(&config);
    if let Some(window) = app_handle.get_webview_window("main") {
        if let Ok(hwnd) = window.hwnd() {
            WindowHelper::position_overlay(
                hwnd.0 as isize,
                &config.position,
                380,
                130,
                config.margin_x,
                config.margin_y,
            );
            WindowHelper::set_click_through(hwnd.0 as isize, config.click_through);
        }
    }
}

#[tauri::command]
fn set_click_through(app_handle: AppHandle, enabled: bool) {
    let mut config = ConfigManager::load();
    config.click_through = enabled;
    ConfigManager::save(&config);
    if let Some(window) = app_handle.get_webview_window("main") {
        if let Ok(hwnd) = window.hwnd() {
            WindowHelper::set_click_through(hwnd.0 as isize, enabled);
        }
    }
}

#[tauri::command]
fn set_position(app_handle: AppHandle, position: String) {
    let mut config = ConfigManager::load();
    config.position = position.clone();
    ConfigManager::save(&config);
    if let Some(window) = app_handle.get_webview_window("main") {
        if let Ok(hwnd) = window.hwnd() {
            WindowHelper::position_overlay(
                hwnd.0 as isize,
                &position,
                380,
                130,
                config.margin_x,
                config.margin_y,
            );
        }
    }
}

#[tauri::command]
fn set_theme(theme: String) {
    let mut config = ConfigManager::load();
    config.theme = theme;
    ConfigManager::save(&config);
}

#[tauri::command]
fn open_themes_folder() {
    let themes_dir = ConfigManager::get_themes_dir();
    let _ = std::process::Command::new("explorer").arg(themes_dir).spawn();
}

pub fn run() {
    let audio_engine = AudioEngine::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            audio_engine: Mutex::new(audio_engine),
        })
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Setup system tray
            if let Err(e) = tray::setup_tray(&app_handle) {
                eprintln!("Failed to setup tray: {:?}", e);
            }

            // Position overlay above taskbar
            let config = ConfigManager::load();
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(hwnd) = window.hwnd() {
                    WindowHelper::position_overlay(
                        hwnd.0 as isize,
                        &config.position,
                        380,
                        130,
                        config.margin_x,
                        config.margin_y,
                    );
                    if config.click_through {
                        WindowHelper::set_click_through(hwnd.0 as isize, true);
                    }
                }
            }

            // Start Audio Engine polling loop
            let state = app.state::<AppState>();
            state.audio_engine.lock().unwrap().start_polling(app_handle);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            set_click_through,
            set_position,
            set_theme,
            open_themes_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running MusicMotion application");
}
