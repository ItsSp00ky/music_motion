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
        WindowHelper::position_overlay(
            &window,
            &config.position,
            380.0,
            130.0,
            config.margin_x as f64,
            config.margin_y as f64,
        );
        WindowHelper::set_click_through(&window, config.click_through);
    }
}

#[tauri::command]
fn set_click_through(app_handle: AppHandle, enabled: bool) {
    let mut config = ConfigManager::load();
    config.click_through = enabled;
    ConfigManager::save(&config);
    if let Some(window) = app_handle.get_webview_window("main") {
        WindowHelper::set_click_through(&window, enabled);
    }
}

#[tauri::command]
fn set_position(app_handle: AppHandle, position: String) {
    let mut config = ConfigManager::load();
    config.position = position.clone();
    ConfigManager::save(&config);
    if let Some(window) = app_handle.get_webview_window("main") {
        WindowHelper::position_overlay(
            &window,
            &position,
            380.0,
            130.0,
            config.margin_x as f64,
            config.margin_y as f64,
        );
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
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("explorer").arg(themes_dir).spawn();
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(themes_dir).spawn();
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(themes_dir).spawn();
}

pub fn run() {
    #[cfg(target_os = "linux")]
    {
        if std::env::var("GDK_BACKEND").is_err() {
            std::env::set_var("GDK_BACKEND", "x11");
        }
    }

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

            // Position overlay above taskbar / screen edge
            let config = ConfigManager::load();
            if let Some(window) = app.get_webview_window("main") {
                WindowHelper::position_overlay(
                    &window,
                    &config.position,
                    380.0,
                    130.0,
                    config.margin_x as f64,
                    config.margin_y as f64,
                );
                if config.click_through {
                    WindowHelper::set_click_through(&window, true);
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
