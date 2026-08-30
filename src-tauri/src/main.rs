// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    {
        // On Wayland compositors (GNOME Mutter, etc.), native xdg_toplevel prohibits
        // client-side window positioning (gtk_window_move) and always-on-top keep-above.
        // Forcing GDK_BACKEND=x11 runs under XWayland/X11 where EWMH overlay hints,
        // exact corner positioning, and topmost HUD layering are fully supported.
        if std::env::var("GDK_BACKEND").is_err() {
            std::env::set_var("GDK_BACKEND", "x11");
        }
    }
    music_motion_lib::run();
}
