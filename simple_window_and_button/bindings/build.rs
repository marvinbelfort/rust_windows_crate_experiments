fn main() {
    windows::build! {
        Windows::Win32::Foundation::*,
        Windows::Win32::Graphics::Gdi::*,
        Windows::Win32::System::LibraryLoader::*,
        Windows::Win32::System::Threading::GetCurrentThreadId,
        Windows::Win32::UI::HiDpi::SetProcessDpiAwareness,
        Windows::Win32::UI::KeyboardAndMouseInput::SetFocus,
        Windows::Win32::UI::WindowsAndMessaging::*,
    };
}