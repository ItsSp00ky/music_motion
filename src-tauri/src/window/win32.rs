use tauri::{LogicalPosition, LogicalSize, Position, Size, WebviewWindow};

pub struct Win32WindowHelper;

impl Win32WindowHelper {
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
        (1920.0, 1080.0)
    }

    pub fn setup_window(window: &WebviewWindow) {
        let _ = window.set_always_on_top(true);
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::WindowsAndMessaging::{
                GetWindowLongPtrW, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_TOPMOST,
                SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, WS_EX_NOACTIVATE,
                WS_EX_TOOLWINDOW, WS_EX_TOPMOST,
            };

            if let Ok(hwnd_ptr) = window.hwnd() {
                let hwnd = HWND(hwnd_ptr.0 as _);
                unsafe {
                    let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                    SetWindowLongPtrW(
                        hwnd,
                        GWL_EXSTYLE,
                        ex_style | (WS_EX_TOPMOST.0 | WS_EX_TOOLWINDOW.0 | WS_EX_NOACTIVATE.0) as isize,
                    );
                    let _ = SetWindowPos(
                        hwnd,
                        HWND_TOPMOST,
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
                    );
                }
            }
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
            let (w, h) = Self::get_work_area();
            (0.0, 0.0, w, h, 1.0)
        };

        let (x, y) = match position {
            "bottom-left" => (mon_x + margin_x, mon_y + screen_h - height - margin_y),
            "top-right" => (mon_x + screen_w - width - margin_x, mon_y + margin_y),
            "top-left" => (mon_x + margin_x, mon_y + margin_y),
            _ => (mon_x + screen_w - width - margin_x, mon_y + screen_h - height - margin_y), // default bottom-right
        };

        eprintln!(
            "[MusicMotion Windows] Positioning overlay at x={}, y={}, size={}x{}, scale={}",
            x, y, width, height, scale
        );

        let _ = window.set_size(Size::Logical(LogicalSize { width, height }));
        let _ = window.set_position(Position::Logical(LogicalPosition { x, y }));
        let _ = window.set_always_on_top(true);
        let _ = window.show();

        #[cfg(target_os = "windows")]
        Self::setup_window(window);
    }

    pub fn set_click_through(window: &WebviewWindow, enabled: bool) {
        let _ = window.set_ignore_cursor_events(enabled);
    }

    pub fn reassert_topmost(window: &WebviewWindow) {
        let _ = window.set_always_on_top(true);
        #[cfg(target_os = "windows")]
        Self::setup_window(window);
    }
}
