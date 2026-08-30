use tauri::{LogicalPosition, LogicalSize, Position, Size, WebviewWindow};
#[cfg(target_os = "linux")]
use gtk::prelude::*;

pub struct LinuxWindowHelper;

impl LinuxWindowHelper {
    pub fn setup_window(window: &WebviewWindow) {
        #[cfg(target_os = "linux")]
        if let Ok(gtk_win) = window.gtk_window() {
            gtk_win.set_decorated(false);
            gtk_win.set_resizable(false);
            gtk_win.set_skip_taskbar_hint(true);
            gtk_win.set_skip_pager_hint(true);
            gtk_win.set_keep_above(true);
            gtk_win.stick(); // Stays visible across all virtual desktops / workspaces

            // Utility / Dock type hint tells GNOME Mutter, KDE KWin, and EWMH window managers
            // that this window is an overlay HUD and must stay above fullscreen and regular apps
            gtk_win.set_type_hint(gdk::WindowTypeHint::Utility);

            // Prevent overlay from stealing focus from active fullscreen browsers/games
            gtk_win.set_accept_focus(false);
        }

        let _ = window.set_always_on_top(true);
        let _ = window.set_visible_on_all_workspaces(true);
    }

    pub fn position_overlay(
        window: &WebviewWindow,
        position: &str,
        width: f64,
        height: f64,
        margin_x: f64,
        margin_y: f64,
    ) {
        Self::setup_window(window);

        let monitor_opt = window
            .current_monitor()
            .ok()
            .flatten()
            .or_else(|| window.primary_monitor().ok().flatten())
            .or_else(|| window.available_monitors().ok().and_then(|m| m.into_iter().next()));

        let (mon_x, mon_y, screen_w, screen_h, scale) = if let Some(monitor) = monitor_opt {
            let scale = monitor.scale_factor();
            let size = monitor.size();
            let pos = monitor.position();
            (
                pos.x as f64 / scale,
                pos.y as f64 / scale,
                size.width as f64 / scale,
                size.height as f64 / scale,
                scale,
            )
        } else {
            (0.0, 0.0, 1920.0, 1080.0, 1.0)
        };

        let (x, y) = match position {
            "bottom-left" => (mon_x + margin_x, mon_y + screen_h - height - margin_y),
            "top-right" => (mon_x + screen_w - width - margin_x, mon_y + margin_y),
            "top-left" => (mon_x + margin_x, mon_y + margin_y),
            _ => (mon_x + screen_w - width - margin_x, mon_y + screen_h - height - margin_y), // default bottom-right
        };

        eprintln!(
            "[MusicMotion Linux] Positioning overlay at x={}, y={}, size={}x{}, scale={}",
            x, y, width, height, scale
        );

        let _ = window.set_size(Size::Logical(LogicalSize { width, height }));
        let _ = window.set_position(Position::Logical(LogicalPosition { x, y }));

        #[cfg(target_os = "linux")]
        if let Ok(gtk_win) = window.gtk_window() {
            gtk_win.set_keep_above(true);
            gtk_win.stick();
            gtk_win.move_(x as i32, y as i32);
        }

        let _ = window.set_always_on_top(true);
        let _ = window.set_visible_on_all_workspaces(true);
        let _ = window.show();
    }

    pub fn set_click_through(window: &WebviewWindow, enabled: bool) {
        let _ = window.set_ignore_cursor_events(enabled);
        #[cfg(target_os = "linux")]
        if let Ok(gtk_win) = window.gtk_window() {
            if enabled {
                gtk_win.set_accept_focus(false);
            }
        }
    }

    pub fn reassert_topmost(window: &WebviewWindow) {
        let _ = window.set_always_on_top(true);
        #[cfg(target_os = "linux")]
        if let Ok(gtk_win) = window.gtk_window() {
            gtk_win.set_keep_above(true);
        }
    }
}
