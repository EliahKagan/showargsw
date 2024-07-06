//! Displays numbered command-line arguments in a Windows dialog box.

use windows::{
    core::{w, Error, PCWSTR},
    Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
    Win32::System::LibraryLoader::LoadLibraryW,
    Win32::UI::Controls::{
        InitCommonControlsEx, TaskDialog, ICC_STANDARD_CLASSES, INITCOMMONCONTROLSEX,
        TDCBF_CLOSE_BUTTON, TD_INFORMATION_ICON,
    },
};

/// Allow COM functionality to be used on this thread.
/// FIXME: This needs a matching CoUninitialize() call!
fn initialize_com() -> Result<(), Error> {
    let result = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    if result.is_err() {
        Err(Error::from_hresult(result))
    } else {
        Ok(())
    }
}

/// Performs initialization needed to create common controls such as task dialogs.
fn initialize_common_controls() {
    let icc = INITCOMMONCONTROLSEX {
        dwSize: std::mem::size_of::<INITCOMMONCONTROLSEX>() as u32,
        dwICC: ICC_STANDARD_CLASSES,
    };
    let result = unsafe { InitCommonControlsEx(&icc) };
    if !result.as_bool() {
        panic!("Failed to initialize common controls!");
    }
}

/// Displays a simple informational task dialog box, similar to a message box.
fn display_task_dialog(
    window_title: PCWSTR,
    main_instruction: PCWSTR,
    content: &[u16],
) -> Result<(), Error> {
    let pszcontent = PCWSTR(content.as_ptr());

    unsafe {
        TaskDialog(
            None,
            None,
            window_title,
            main_instruction,
            pszcontent,
            TDCBF_CLOSE_BUTTON,
            TD_INFORMATION_ICON,
            None,
        )?;
    };

    Ok(())
}

fn main() -> Result<(), Error> {
    unsafe {
        LoadLibraryW(w!("ComCtl32.dll")).expect("need COM dll");
    }

    initialize_com().expect("initializing COM should succeed");
    initialize_common_controls();

    let lines: Vec<String> = std::env::args()
        .enumerate()
        .map(|(i, arg)| format!("{i}: [{arg}]"))
        .collect();

    let content: Vec<u16> = lines
        .join("\n")
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    display_task_dialog(
        w!("showargsw"),
        w!("The following command-line arguments were passed."),
        &content,
    )
}
