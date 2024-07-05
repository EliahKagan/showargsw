use std::{env, iter};

use windows::{
    core::{w, PCWSTR},
    Win32::UI::WindowsAndMessaging::{MessageBoxW, SetProcessDPIAware, MB_ICONINFORMATION, MB_OK},
};

fn main() {
    let lines: Vec<String> = env::args()
        .enumerate()
        .map(|(i, arg)| format!("{i}: [{arg}]"))
        .collect();
    let message: Vec<u16> = lines
        .join("\n")
        .encode_utf16()
        .chain(iter::once(0))
        .collect();

    unsafe {
        if !SetProcessDPIAware().as_bool() {
            eprintln!("Couldn't respect display scaling!");
        }
        MessageBoxW(
            None,
            PCWSTR(message.as_ptr()),
            w!("Command-line arguments"),
            MB_OK | MB_ICONINFORMATION,
        );
    }
}
