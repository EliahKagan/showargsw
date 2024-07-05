//! Displays numbered command-line arguments in a Windows message box.

use windows::{
    core::{w, Error, PCWSTR},
    Win32::UI::WindowsAndMessaging::{
        MessageBoxW, SetProcessDPIAware, MB_ICONINFORMATION, MB_OK, MESSAGEBOX_RESULT,
    },
};

/// The text may still be too small, but this at least keeps it from being blurry.
fn attempt_dpi_awareness() {
    let dpi_aware = unsafe { SetProcessDPIAware() };

    if !dpi_aware.as_bool() {
        eprintln!("Couldn't respect display scaling!");
    }
}

/// Shows an informational message box.
fn display_message_box(message: &[u16], title: PCWSTR) -> Result<(), Error> {
    let lptext = PCWSTR(message.as_ptr());
    let style = MB_OK | MB_ICONINFORMATION;

    let mb_result = unsafe { MessageBoxW(None, lptext, title, style) };

    match mb_result {
        MESSAGEBOX_RESULT(0) => Err(Error::from_win32()),
        _ => Ok(()),
    }
}

fn main() -> Result<(), Error> {
    attempt_dpi_awareness();

    let lines: Vec<String> = std::env::args()
        .enumerate()
        .map(|(i, arg)| format!("{i}: [{arg}]"))
        .collect();

    let message: Vec<u16> = lines
        .join("\n")
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    display_message_box(&message, w!("Command-line arguments"))
}
