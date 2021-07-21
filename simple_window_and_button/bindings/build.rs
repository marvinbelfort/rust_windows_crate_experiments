fn main() {
    windows::build! {
        Windows::Win32::Foundation::{HWND, PSTR, HINSTANCE, WPARAM, LPARAM, LRESULT},
        Windows::Win32::Graphics::Gdi::HBRUSH,
        Windows::Win32::System::LibraryLoader::GetModuleHandleA,
        Windows::Win32::UI::WindowsAndMessaging::{
            CreateWindowExA, DefWindowProcA, DestroyWindow, DispatchMessageA, GetMessageA, 
            GetWindowLongPtrA, PostQuitMessage, RegisterClassA, TranslateMessage, CW_USEDEFAULT,
            WM_DESTROY, WM_COMMAND, WM_CREATE, WM_QUIT
        }
    };
}