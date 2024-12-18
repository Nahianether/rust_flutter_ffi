#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
use std::ptr::null_mut;
#[cfg(windows)]
use winapi::shared::windef::{HHOOK, HWND};
#[cfg(windows)]
use winapi::um::errhandlingapi::GetLastError;
#[cfg(windows)]
use winapi::um::winuser::{
    CallNextHookEx, DestroyWindow, GetClassNameW, SetWindowsHookExW, UnhookWindowsHookEx, WH_CBT,
    HCBT_CREATEWND,
};
#[cfg(windows)]
use winapi::um::processthreadsapi::GetCurrentThreadId;

#[cfg(windows)]
static mut HOOK_HANDLE: Option<HHOOK> = None;

#[cfg(windows)]
unsafe extern "system" fn hook_proc(n_code: i32, w_param: usize, _l_param: isize) -> isize {
    if n_code == HCBT_CREATEWND {
        let hwnd: HWND = w_param as HWND;

        // Get the class name of the window being created
        let mut class_name = vec![0u16; 256];
        if GetClassNameW(hwnd, class_name.as_mut_ptr(), class_name.len() as i32) > 0 {
            let class_name = String::from_utf16_lossy(
                &class_name[..class_name.iter().position(|&c| c == 0).unwrap_or(class_name.len())],
            );

            // Debugging: Print detected class names
            println!("Detected class name: {}", class_name);

            // Check if the dialog is part of Google Chrome
            if class_name.contains("Chrome_WidgetWin_1") || class_name.contains("FileDialog") {
                // Block the window by destroying it
                DestroyWindow(hwnd);
                println!("Blocked a file picker dialog.");
                return 1; // Prevent the window from proceeding
            }
        }
    }
    CallNextHookEx(null_mut(), n_code, w_param, _l_param)
}

#[cfg(windows)]
pub fn start_blocking_file_picker() {
    unsafe {
        let hook = SetWindowsHookExW(
            WH_CBT,
            Some(hook_proc),
            null_mut(),
            GetCurrentThreadId(),
        );

        if !hook.is_null() {
            println!("Blocking file picker dialogs started.");
            HOOK_HANDLE = Some(hook);
        } else {
            let error_code = GetLastError();
            eprintln!("Failed to set the hook. Error code: {}", error_code);
        }
    }
}

#[cfg(windows)]
pub fn stop_blocking_file_picker() {
    unsafe {
        if let Some(hook) = HOOK_HANDLE {
            UnhookWindowsHookEx(hook);
            println!("Blocking file picker dialogs stopped.");
            HOOK_HANDLE = None;
        } else {
            eprintln!("No hook is currently set.");
        }
    }
}

// fn main() {
//     println!("Starting to block file picker dialogs...");
//     start_blocking_file_picker();

//     // Keep the program running to maintain the hook
//     loop {
//         std::thread::sleep(std::time::Duration::from_secs(1));
//     }

//     stop_blocking_file_picker();
// }
