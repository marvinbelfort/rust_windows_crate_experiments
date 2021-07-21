use bindings::{
    Windows::Win32::Foundation::*,
    Windows::Win32::Graphics::Gdi::*,
    Windows::Win32::System::LibraryLoader::*,
    Windows::Win32::UI::WindowsAndMessaging::{
        CreateWindowExA, DefWindowProcA, DestroyWindow, DispatchMessageA, GetMessageA,
        GetWindowLongPtrA, PostQuitMessage, RegisterClassA, TranslateMessage, CS_HREDRAW,
        CS_VREDRAW, CW_USEDEFAULT, GWLP_HINSTANCE, HMENU, MSG, WINDOW_EX_STYLE, WM_COMMAND,
        WM_CREATE, WM_DESTROY, WM_QUIT, WNDCLASSA, WS_CHILD, WS_EX_OVERLAPPEDWINDOW,
        WS_OVERLAPPEDWINDOW, WS_TABSTOP, WS_VISIBLE,
    },
};
use std::ffi::CString;

fn create_button(pwnd: HWND, control_id: isize) {
    let class_name = CString::new("BUTTON").expect("BUTTON");
    let pstr_class_name: PSTR = PSTR(class_name.as_ptr() as *mut _);

    let wnd_name = CString::new("Press Me!").expect("BUTTON");
    let pstr_wnd_name: PSTR = PSTR(wnd_name.as_ptr() as *mut _);

    let control_id_as_hmenu = HMENU(control_id);

    unsafe {
        CreateWindowExA(
            WINDOW_EX_STYLE(0),
            pstr_class_name,
            pstr_wnd_name,
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            5,
            5,
            100,
            30,
            pwnd,
            control_id_as_hmenu,
            HINSTANCE(GetWindowLongPtrA(pwnd, GWLP_HINSTANCE)),
            std::ptr::null_mut(),
        );
    }
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            create_button(hwnd, 101);
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        WM_QUIT => {
            unsafe {
                DestroyWindow(hwnd);
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let loword = w_param.0 & 0xff;
            match loword {
                101 => {
                    println!("Button was pressed!");
                    LRESULT(0)
                }

                _ => LRESULT(0),
            }
        }
        _ => unsafe { DefWindowProcA(hwnd, msg, w_param, l_param) },
    }
}

fn main() -> windows::Result<()> {
    unsafe {
        let h_brush: HBRUSH = HBRUSH(6);
        let class_name = std::ffi::CString::new("RUST_WND_CLASS").expect("CString::new failed");
        let pstr_class_name: PSTR = PSTR(class_name.as_ptr() as *mut _);

        let window_name = std::ffi::CString::new("Rust Window").expect("CString::new failed");
        let pstr_window_name: PSTR = PSTR(window_name.as_ptr() as *mut _);

        let wc = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            lpszClassName: pstr_class_name,
            hbrBackground: h_brush,
            ..WNDCLASSA::default()
        };

        let result = RegisterClassA(&wc);
        println!("register class = {:#X}", result);

        let _h_wnd = CreateWindowExA(
            WS_EX_OVERLAPPEDWINDOW,
            pstr_class_name,
            pstr_window_name,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            640,
            480,
            HWND::NULL,
            HMENU::NULL,
            GetModuleHandleA(None),
            std::ptr::null_mut(),
        );

        let mut msg = MSG::default();
        let h_wnd2 = HWND::default();

        loop {
            let result = GetMessageA(&mut msg, h_wnd2, 0, 0).0;
            if result < 1 {
                break;
            } else {
                match msg.message {
                    WM_QUIT => (),
                    _ => {
                        TranslateMessage(&msg);
                        DispatchMessageA(&msg);
                    }
                }
            }
        }
    }

    Ok(())
}

// ref: https://github.com/iFuSiiOnzZ/SoundCloud-Downloader/blob/f4f4c27d6d470a7603a57a3c62b68905635e01c1/src/platform/windows/window.cpp
