use windows::Win32::UI::WindowsAndMessaging::{
    MB_OK,
    MessageBoxW,
};
use windows::core::w;

fn main() {
    unsafe {
        MessageBoxW(
            None,
            w!("Hello, world!"),
            w!("Greetings"),
            MB_OK,
        );
    }
}
