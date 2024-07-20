//! Displays numbered command-line arguments in a Windows message box.

use windows::{
    core::{h, Error, HSTRING},
    Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK, MESSAGEBOX_RESULT},
};

fn main() -> Result<(), Error> {
    let text = std::env::args()
        .enumerate()
        .map(|(i, arg)| format!("{i}: [{arg}]"))
        .collect::<Vec<_>>()
        .join("\n");

    display_message_box(&text.into(), h!("Command-line arguments"))
}

/// Shows an informational message box.
fn display_message_box(text: &HSTRING, caption: &HSTRING) -> Result<(), Error> {
    let style = MB_OK | MB_ICONINFORMATION;
    let result = unsafe { MessageBoxW(None, text, caption, style) };
    match result {
        MESSAGEBOX_RESULT(0) => Err(Error::from_win32()),
        _ => Ok(()),
    }
}
