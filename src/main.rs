//! Displays numbered command-line arguments in a Windows message box.

use windows::{
    core::{h, Error, HSTRING},
    Win32::UI::WindowsAndMessaging::{
        MessageBoxW, SetProcessDPIAware, MB_ICONINFORMATION, MB_OK, MESSAGEBOX_RESULT,
    },
};

fn main() -> Result<(), Error> {
    attempt_dpi_awareness();

    let message = std::env::args()
        .enumerate()
        .map(|(i, arg)| format!("{i}: [{arg}]"))
        .collect::<Vec<_>>()
        .join("\n");

    display_message_box(&message.into(), h!("Command-line arguments"))
}

/// The text may still be too small, but this at least keeps it from being blurry.
fn attempt_dpi_awareness() {
    let dpi_aware = unsafe { SetProcessDPIAware() };

    if !dpi_aware.as_bool() {
        eprintln!("Couldn't respect display scaling!");
    }
}

/// Shows an informational message box.
fn display_message_box(message: &HSTRING, title: &HSTRING) -> Result<(), Error> {
    let style = MB_OK | MB_ICONINFORMATION;

    let result = unsafe { MessageBoxW(None, message, title, style) };

    match result {
        MESSAGEBOX_RESULT(0) => Err(Error::from_win32()),
        _ => Ok(()),
    }
}
