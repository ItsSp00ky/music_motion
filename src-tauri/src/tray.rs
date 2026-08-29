use crate::config::{AppConfig, ConfigManager};
use crate::window::WindowHelper;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigManager::load();

    let title_item = MenuItem::with_id(app, "title", "🎵 MusicMotion", false, None::<&str>)?;
    let sep1 = PredefinedMenuItem::separator(app)?;

    let click_through_item = CheckMenuItem::with_id(
        app,
        "toggle_click_through",
        "Click-Through Mode (Pass Clicks)",
        true,
        config.click_through,
        None::<&str>,
    )?;

    // Position Submenu
    let pos_br = MenuItem::with_id(app, "pos_br", "Bottom-Right (Default)", true, None::<&str>)?;
    let pos_bl = MenuItem::with_id(app, "pos_bl", "Bottom-Left", true, None::<&str>)?;
    let pos_tr = MenuItem::with_id(app, "pos_tr", "Top-Right", true, None::<&str>)?;
    let pos_tl = MenuItem::with_id(app, "pos_tl", "Top-Left", true, None::<&str>)?;
    let pos_submenu = Submenu::with_items(
        app,
        "Screen Position",
        true,
        &[&pos_br, &pos_bl, &pos_tr, &pos_tl],
    )?;

    // Theme Submenu
    let theme_frosted = MenuItem::with_id(app, "theme_frosted", "Frosted Fluent Card", true, None::<&str>)?;
    let theme_minimal = MenuItem::with_id(app, "theme_minimal", "Minimal HUD", true, None::<&str>)?;
    let theme_island = MenuItem::with_id(app, "theme_island", "Dynamic Island", true, None::<&str>)?;
    let theme_neon = MenuItem::with_id(app, "theme_neon", "Cyber Neon", true, None::<&str>)?;
    let theme_submenu = Submenu::with_items(
        app,
        "Themes",
        true,
        &[&theme_frosted, &theme_minimal, &theme_island, &theme_neon],
    )?;

    let open_themes = MenuItem::with_id(app, "open_themes", "Open Themes Folder...", true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit MusicMotion", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &title_item,
            &sep1,
            &click_through_item,
            &pos_submenu,
            &theme_submenu,
            &open_themes,
            &sep2,
            &quit_item,
        ],
    )?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("MusicMotion - Audio Overlay")
        .on_menu_event(move |app_handle, event| {
            let id = event.id().as_ref();
            let mut current_config = ConfigManager::load();

            match id {
                "toggle_click_through" => {
                    current_config.click_through = !current_config.click_through;
                    ConfigManager::save(&current_config);
                    if let Some(window) = app_handle.get_webview_window("main") {
                        if let Ok(hwnd) = window.hwnd() {
                            WindowHelper::set_click_through(hwnd.0 as isize, current_config.click_through);
                        }
                        let _ = window.emit("config-update", current_config.clone());
                    }
                }
                "pos_br" => apply_position(app_handle, &mut current_config, "bottom-right"),
                "pos_bl" => apply_position(app_handle, &mut current_config, "bottom-left"),
                "pos_tr" => apply_position(app_handle, &mut current_config, "top-right"),
                "pos_tl" => apply_position(app_handle, &mut current_config, "top-left"),
                "theme_frosted" => apply_theme(app_handle, &mut current_config, "frosted-card"),
                "theme_minimal" => apply_theme(app_handle, &mut current_config, "minimal-hud"),
                "theme_island" => apply_theme(app_handle, &mut current_config, "dynamic-island"),
                "theme_neon" => apply_theme(app_handle, &mut current_config, "cyber-neon"),
                "open_themes" => {
                    let themes_dir = ConfigManager::get_themes_dir();
                    let _ = std::process::Command::new("explorer").arg(themes_dir).spawn();
                }
                "quit" => {
                    app_handle.exit(0);
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}

fn apply_position(app: &AppHandle, config: &mut AppConfig, pos: &str) {
    config.position = pos.to_string();
    ConfigManager::save(config);
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(hwnd) = window.hwnd() {
            WindowHelper::position_overlay(hwnd.0 as isize, pos, 380, 130, config.margin_x, config.margin_y);
        }
        let _ = window.emit("config-update", config.clone());
    }
}

fn apply_theme(app: &AppHandle, config: &mut AppConfig, theme: &str) {
    config.theme = theme.to_string();
    ConfigManager::save(config);
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.emit("config-update", config.clone());
    }
}
