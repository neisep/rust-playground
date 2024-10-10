use std::ptr::null_mut;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::commctrl;
use winapi::um::wingdi::TextOutW;
use winapi::um::winuser::COLOR_WINDOW;
use winapi::shared::windef::{HBRUSH, HDC, HWND, RECT};
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};

extern crate winapi;

use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, PostQuitMessage,
    RegisterClassW, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW,
    MSG, SW_SHOW, WM_DESTROY, WM_PAINT, WNDCLASSW, WS_OVERLAPPEDWINDOW,
};

fn to_wstring(str: &str) -> Vec<u16> {
    str.encode_utf16().chain(Some(0)).collect()
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = std::mem::zeroed();
            let hdc: HDC = winapi::um::winuser::BeginPaint(hwnd, &mut ps);
            let text = to_wstring("Hello, World!");
            let mut rect: RECT = std::mem::zeroed();
            winapi::um::winuser::GetClientRect(hwnd, &mut rect);
            TextOutW(hdc, 50, 50, text.as_ptr(), text.len() as i32);
            winapi::um::winuser::EndPaint(hwnd, &ps);
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn main() {
    unsafe {
        let h_instance: HINSTANCE = GetModuleHandleW(null_mut());
        let class_name = to_wstring("my_window");

        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance,
            lpszClassName: class_name.as_ptr(),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
            ..std::mem::zeroed()
        };

        RegisterClassW(&wnd_class);

        let hwnd: HWND = CreateWindowExW(
            0,
            class_name.as_ptr(),
            to_wstring("Hello, World! Window").as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            null_mut(),
            null_mut(),
            h_instance,
            null_mut(),
        );

        ShowWindow(hwnd, SW_SHOW);

        // Create a button with the text "Start export"
        let button_text = to_wstring("Start export");
        CreateWindowExW(
            0,
            to_wstring("BUTTON").as_ptr(),
            button_text.as_ptr(),
            winapi::um::winuser::WS_CHILD | winapi::um::winuser::WS_VISIBLE | winapi::um::winuser::BS_DEFPUSHBUTTON,
            50, // x position
            100, // y position
            150, // width
            50, // height
            hwnd,
            null_mut(),
            h_instance,
            null_mut(),
        );
        // Create a progress bar
        let progress_bar_class = to_wstring("msctls_progress32");
        let progress_bar_hwnd = CreateWindowExW(
            0,
            progress_bar_class.as_ptr(),
            null_mut(),
            winapi::um::winuser::WS_CHILD | winapi::um::winuser::WS_VISIBLE,
            50, // x position
            200, // y position
            700, // width
            30, // height
            hwnd,
            null_mut(),
            h_instance,
            null_mut(),
        );

        // Set the range and initial position of the progress bar
        //winapi::um::commctrl::SendMessageW(progress_bar_hwnd, winapi::um::commctrl::PBM_SETRANGE, 0, (0 << 16) | 100);
        //winapi::um::commctrl::SendMessageW(progress_bar_hwnd, winapi::um::commctrl::PBM_SETPOS, 0, 0);

        // Create a panel
        let panel_hwnd = CreateWindowExW(
            0,
            to_wstring("STATIC").as_ptr(),
            null_mut(),
            winapi::um::winuser::WS_CHILD | winapi::um::winuser::WS_VISIBLE | winapi::um::winuser::SS_WHITERECT,
            50, // x position
            300, // y position
            200, // width
            100, // height
            hwnd,
            null_mut(),
            h_instance,
            null_mut(),
        );

        // Create a button inside the panel with the text "Click me"
        let button_text = to_wstring("Click me");
        CreateWindowExW(
            0,
            to_wstring("BUTTON").as_ptr(),
            button_text.as_ptr(),
            winapi::um::winuser::WS_CHILD | winapi::um::winuser::WS_VISIBLE | winapi::um::winuser::BS_DEFPUSHBUTTON,
            25, // x position relative to panel
            25, // y position relative to panel
            150, // width
            50, // height
            panel_hwnd,
            null_mut(),
            h_instance,
            null_mut(),
        );

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}