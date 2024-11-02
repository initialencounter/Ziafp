use windows::core::s;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, ShowWindow, SW_HIDE};

pub fn hide_console_window() {
    unsafe {
        let window = FindWindowA(s!("ConsoleWindowClass"), None).unwrap_or(HWND(0 as isize as _));
        if window != HWND(0 as isize as _) {
            let _ = ShowWindow(window, SW_HIDE);
        }
    }
}
