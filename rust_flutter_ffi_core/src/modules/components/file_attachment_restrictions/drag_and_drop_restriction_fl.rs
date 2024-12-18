#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
use std::ptr;
#[cfg(windows)]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(windows)]
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM};
#[cfg(windows)]
use winapi::shared::windef::HHOOK;
#[cfg(windows)]
use winapi::um::winuser::{
    CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, GetMessageW, WH_MOUSE_LL,
    WM_LBUTTONDOWN, WM_MOUSEMOVE, WM_LBUTTONUP,
};

// Global variables to track drag state and hook handle
#[cfg(windows)]
static mut HOOK_HANDLE: HHOOK = ptr::null_mut();
#[cfg(windows)]
static IS_DRAGGING: AtomicBool = AtomicBool::new(false);

/// Low-level mouse hook procedure
#[cfg(windows)]
unsafe extern "system" fn mouse_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // Ensure the hook chain continues if the code is less than 0
    if code < 0 {
        return CallNextHookEx(HOOK_HANDLE, code, wparam, lparam);
    }

    match wparam as u32 {
        WM_LBUTTONDOWN => {
            // Left mouse button pressed: Start tracking drag
            println!("Left mouse button down detected. Dragging started.");
            IS_DRAGGING.store(true, Ordering::SeqCst);
        }
        WM_MOUSEMOVE => {
            if IS_DRAGGING.load(Ordering::SeqCst) {
                // If dragging, block mouse movement
                println!("Dragging detected. Blocking mouse move.");
                return 1; // Block mouse movement during drag
            }
        }
        WM_LBUTTONUP => {
            // Left mouse button released: Stop tracking drag
            println!("Left mouse button released. Dragging stopped.");
            IS_DRAGGING.store(false, Ordering::SeqCst);
        }
        _ => {}
    }

    // Pass all other events to the next hook in the chain
    CallNextHookEx(HOOK_HANDLE, code, wparam, lparam)
}

/// Function to install the low-level mouse hook
#[cfg(windows)]
pub fn block_drag_and_drop() {
    unsafe {
        // Install the mouse hook
        HOOK_HANDLE = SetWindowsHookExW(
            WH_MOUSE_LL,
            Some(mouse_hook_proc),
            ptr::null_mut() as HINSTANCE,
            0, // 0 for a global hook
        );

        if HOOK_HANDLE.is_null() {
            eprintln!("Failed to install mouse hook.");
            return;
        }

        println!("Drag-and-drop blocking hook installed.");
    }
}

/// Function to uninstall the hook
#[cfg(windows)]
pub fn stop_blocking_drag_and_drop() {
    unsafe {
        if !HOOK_HANDLE.is_null() {
            UnhookWindowsHookEx(HOOK_HANDLE);
            HOOK_HANDLE = ptr::null_mut();
            println!("Drag-and-drop blocking hook uninstalled.");
        }
    }
}

/// Keep the application running with a message loop
#[cfg(windows)]
pub fn message_loop() {
    unsafe {
        let mut msg = std::mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {}
    }
}