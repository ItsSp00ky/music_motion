use tauri::{LogicalPosition, LogicalSize, Position, Size, WebviewWindow};
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPI_GETWORKAREA, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
};

pub struct WindowHelper;

impl WindowHelper {
    pub fn get_work_area() -> RECT {
        unsafe {
            let mut rect = RECT::default();
            let _ = SystemParametersInfoW(
                SPI_GETWORKAREA,
                0,
                Some(&mut rect as *mut _ as *mut _),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            );
            if rect.right == 0 && rect.bottom == 0 {
                rect.right = 1920;
                rect.bottom = 1040;
            }
            rect
        }
    }

    pub fn position_overlay(
        window: &WebviewWindow,
        position: &str,
        width: f64,
        height: f64,
        margin_x: f64,
        margin_y: f64,
    ) {
        let work_area = Self::get_work_area();
        let scale = window.scale_factor().unwrap_or(1.0);

        let screen_w = (work_area.right - work_area.left) as f64 / scale;
        let screen_h = (work_area.bottom - work_area.top) as f64 / scale;

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
