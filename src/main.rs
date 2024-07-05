use windows::Win32::UI::WindowsAndMessaging::{
    MB_OK,
    MessageBoxW,
    SetProcessDPIAware,
};
use windows::core::w;

fn main() {
    unsafe {
        if !SetProcessDPIAware().as_bool() {
            panic!("Couldn't respect display scaling");
        }
        MessageBoxW(
            None,
            w!("Hello, world!"),
            w!("Greetings"),
            MB_OK,
        );
    }
}
