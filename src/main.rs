use windows::Win32::UI::{
    HiDpi::{DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext},
    WindowsAndMessaging::{MB_OK, MessageBoxW},
};
use windows::core::w;

fn main() {
    unsafe {
        if let Err(e) = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) {
            panic!("Couldn't respect display scaling: {:?}", e);
        }
        MessageBoxW(
            None,
            w!("Hello, world!"),
            w!("Greetings"),
            MB_OK,
        );
    }
}
