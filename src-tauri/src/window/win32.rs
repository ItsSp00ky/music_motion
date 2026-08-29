use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongW, SetWindowLongW, SetWindowPos, SystemParametersInfoW, GWL_EXSTYLE,
    HWND_TOPMOST, SPI_GETWORKAREA, SWP_NOACTIVATE, SWP_SHOWWINDOW,
    SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, WS_EX_LAYERED, WS_EX_TOOLWINDOW,
    WS_EX_TRANSPARENT,
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
        hwnd_raw: isize,
        position: &str,
        width: i32,
        height: i32,
        margin_x: i32,
        margin_y: i32,
    ) {
        let hwnd = HWND(hwnd_raw as *mut _);
        let work_area = Self::get_work_area();

        let (x, y) = match position {
            "bottom-left" => (work_area.left + margin_x, work_area.bottom - height - margin_y),
            "top-right" => (work_area.right - width - margin_x, work_area.top + margin_y),
            "top-left" => (work_area.left + margin_x, work_area.top + margin_y),
            _ => (
                work_area.right - width - margin_x,
                work_area.bottom - height - margin_y,
            ), // default "bottom-right"
        };

        eprintln!("[MusicMotion] Positioning overlay at x={}, y={}, w={}, h={}", x, y, width, height);

        unsafe {
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                x,
                y,
                width,
                height,
                SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );
        }
    }

    pub fn set_click_through(hwnd_raw: isize, enabled: bool) {
        let hwnd = HWND(hwnd_raw as *mut _);
        unsafe {
            let current_ex = GetWindowLongW(hwnd, GWL_EXSTYLE);
            let mut new_ex = current_ex | (WS_EX_LAYERED.0 as i32) | (WS_EX_TOOLWINDOW.0 as i32);

            if enabled {
                new_ex |= WS_EX_TRANSPARENT.0 as i32;
            } else {
                new_ex &= !(WS_EX_TRANSPARENT.0 as i32);
            }

            let _ = SetWindowLongW(hwnd, GWL_EXSTYLE, new_ex);
        }
    }
}
