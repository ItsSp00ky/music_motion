use tauri::{LogicalPosition, LogicalSize, Position, Size, WebviewWindow};

pub struct WindowHelper;

impl WindowHelper {
    #[cfg(target_os = "windows")]
    pub fn get_work_area() -> (f64, f64) {
        use windows::Win32::Foundation::RECT;
        use windows::Win32::UI::WindowsAndMessaging::{
            SystemParametersInfoW, SPI_GETWORKAREA, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
        };

        unsafe {
            let mut rect = RECT::default();
            let _ = SystemParametersInfoW(
                SPI_GETWORKAREA,
                0,
                Some(&mut rect as *mut _ as *mut _),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            );
            if rect.right == 0 && rect.bottom == 0 {
                (1920.0, 1040.0)
            } else {
                ((rect.right - rect.left) as f64, (rect.bottom - rect.top) as f64)
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn get_work_area() -> (f64, f64) {
        (1920.0, 1040.0)
    }

    pub fn position_overlay(
        window: &WebviewWindow,
        position: &str,
        width: f64,
        height: f64,
        margin_x: f64,
        margin_y: f64,
    ) {
        let scale = window.scale_factor().unwrap_or(1.0);

        let (screen_w, screen_h) = if let Ok(Some(monitor)) = window.current_monitor() {
            let size = monitor.size();
            (size.width as f64 / scale, size.height as f64 / scale)
        } else {
            let (w, h) = Self::get_work_area();
            (w / scale, h / scale)
        };

        let (x, y) = match position {
            "bottom-left" => (margin_x, screen_h - height - margin_y),
            "top-right" => (screen_w - width - margin_x, margin_y),
            "top-left" => (margin_x, margin_y),
            _ => (screen_w - width - margin_x, screen_h - height - margin_y), // default bottom-right
        };

        eprintln!(
            "[MusicMotion] Window positioned at x={}, y={}, size={}x{}, scale={}",
            x, y, width, height, scale
        );

        let _ = window.set_size(Size::Logical(LogicalSize { width, height }));
        let _ = window.set_position(Position::Logical(LogicalPosition { x, y }));
        let _ = window.set_always_on_top(true);
        let _ = window.show();
    }

    pub fn set_click_through(window: &WebviewWindow, enabled: bool) {
        let _ = window.set_ignore_cursor_events(enabled);
    }
}
