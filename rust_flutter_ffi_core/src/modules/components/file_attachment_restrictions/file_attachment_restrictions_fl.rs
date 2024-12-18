// extern crate winapi;

// use std::ffi::c_void;
// use std::ptr;
// use winapi::shared::windef::HWND;
// use winapi::shared::winerror::{ERROR_CANCELLED, HRESULT, HRESULT_FROM_WIN32};
// use winapi::shared::wtypesbase::CLSCTX_INPROC_SERVER;
// use winapi::um::combaseapi::{CoCreateInstance, CoInitializeEx, CoUninitialize};
// use winapi::um::memoryapi::VirtualProtect;
// use winapi::um::objbase::COINIT_APARTMENTTHREADED;
// use winapi::um::shobjidl::IFileDialog;
// use winapi::um::shobjidl_core::CLSID_FileOpenDialog;
// use winapi::um::winnt::PAGE_EXECUTE_READWRITE;
// use winapi::Interface;

// // Pointer to the original `Show` method
// static mut ORIGINAL_SHOW: Option<unsafe extern "system" fn(*mut IFileDialog, HWND) -> HRESULT> =
//     None;

// // Custom `Show` method implementation
// unsafe extern "system" fn custom_show(this: *mut IFileDialog, hwnd: HWND) -> HRESULT {
//     println!("File picker dialog disabled!");
//     HRESULT_FROM_WIN32(ERROR_CANCELLED)
// }

// // Change vtable memory protection and modify method
// unsafe fn modify_vtable(vtable: *mut *mut c_void, offset: isize, new_func: *mut c_void) -> bool {
//     let method_ptr = (*vtable).offset(offset) as *mut *mut c_void;

//     // Change memory protection to allow modification
//     let mut old_protect: u32 = 0;
//     if VirtualProtect(
//         method_ptr as *mut c_void,
//         std::mem::size_of::<usize>(),
//         PAGE_EXECUTE_READWRITE,
//         &mut old_protect,
//     ) == 0
//     {
//         eprintln!("Failed to change memory protection.");
//         return false;
//     }

//     // Replace the method in the vtable
//     ptr::write(method_ptr, new_func);

//     // Restore the original memory protection
//     VirtualProtect(
//         method_ptr as *mut c_void,
//         std::mem::size_of::<usize>(),
//         old_protect,
//         &mut old_protect,
//     );

//     true
// }

// pub unsafe fn hook_ifiledialog_show() {
//     // Initialize COM
//     let hr = CoInitializeEx(ptr::null_mut(), COINIT_APARTMENTTHREADED);
//     if hr != 0 {
//         eprintln!("Failed to initialize COM: HRESULT = {:#X}", hr);
//         return;
//     }

//     // Create an instance of `IFileDialog`
//     let mut file_dialog: *mut IFileDialog = ptr::null_mut();
//     let hr = CoCreateInstance(
//         &CLSID_FileOpenDialog,
//         ptr::null_mut(),
//         CLSCTX_INPROC_SERVER,
//         &IFileDialog::uuidof(),
//         &mut file_dialog as *mut _ as *mut *mut c_void,
//     );

//     if hr != 0 || file_dialog.is_null() {
//         eprintln!("Failed to create IFileDialog instance: HRESULT = {:#X}", hr);
//         CoUninitialize();
//         return;
//     }

//     // Get the `vtable` of `IFileDialog`
//     let vtable = *(file_dialog as *mut *mut *mut c_void);

//     // Hook the `Show` method in the `vtable`
//     if ORIGINAL_SHOW.is_none() {
//         ORIGINAL_SHOW = Some(ptr::read((*vtable).offset(3)
//             as *mut unsafe extern "system" fn(*mut IFileDialog, HWND) -> HRESULT));
//     }

//     if !modify_vtable(vtable, 3, custom_show as *mut c_void) {
//         eprintln!("Failed to hook the Show method.");
//         (*file_dialog).Release();
//         CoUninitialize();
//         return;
//     }

//     println!("Successfully hooked IFileDialog::Show");

//     // Release the `IFileDialog` instance
//     (*file_dialog).Release();

//     // Uninitialize COM
//     CoUninitialize();
//     println!("COM uninitialized.");
// }

// ---------------------------------------------------------------------------------------------------------------------------

#[cfg(windows)]
extern crate lazy_static;
#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
use lazy_static::lazy_static;
#[cfg(windows)]
use std::ffi::c_void;
#[cfg(windows)]
use std::ptr::null_mut;
#[cfg(windows)]
use std::sync::{Arc, Mutex};
#[cfg(windows)]
use winapi::shared::windef::HWND;
#[cfg(windows)]
use winapi::um::winuser::{
    CallNextHookEx, DestroyWindow, GetClassNameW, SetWindowsHookExW, UnhookWindowsHookEx,
    HCBT_CREATEWND, WH_CBT,
};

#[cfg(windows)]
use std::sync::atomic::AtomicPtr;

#[cfg(windows)]
lazy_static! {
    static ref HOOK_HANDLE: Arc<Mutex<Option<AtomicPtr<c_void>>>> = Arc::new(Mutex::new(None));
}

#[cfg(windows)]
fn set_hook_handle(handle: *mut c_void) {
    println!("Setting hook handle: {:?}", handle);
    let mut hook_handle = HOOK_HANDLE.lock().unwrap();
    *hook_handle = Some(AtomicPtr::new(handle));
    println!("Hook handle set: {:?}", handle);
}

#[cfg(windows)]
fn get_hook_handle() -> Option<*mut c_void> {
    println!("Getting hook handle.");
    let hook_handle = HOOK_HANDLE.lock().unwrap();
    hook_handle
        .as_ref()
        .map(|atomic_ptr| atomic_ptr.load(std::sync::atomic::Ordering::SeqCst))
}

#[cfg(windows)]
unsafe extern "system" fn hook_proc(n_code: i32, w_param: usize, l_param: isize) -> isize {
    if n_code == HCBT_CREATEWND {
        let hwnd: HWND = w_param as HWND;

        // Get the class name of the window being created
        let mut class_name = vec![0u16; 256];
        if GetClassNameW(hwnd, class_name.as_mut_ptr(), class_name.len() as i32) > 0 {
            let class_name = String::from_utf16_lossy(&class_name);

            if class_name.contains("Chrome_WidgetWin_1") {
                // Block the window by destroying it
                DestroyWindow(hwnd);
                println!("Blocked a Google Chrome popup.");
                return 1; // Prevent the window from proceeding
            }
        }
    }
    CallNextHookEx(null_mut(), n_code, w_param, l_param)
}

#[cfg(windows)]
pub fn start_blocking_chrome() {
    unsafe {
        let hook = SetWindowsHookExW(
            WH_CBT,
            Some(hook_proc),
            null_mut(),
            winapi::um::processthreadsapi::GetCurrentThreadId(),
        );
        if !hook.is_null() {
            println!("Google Chrome blocking started.");
            set_hook_handle(hook as *mut c_void);
            println!("Running message loop in the same thread after call set_hook_handle.");

            // Run a message loop to process the hook events in the same thread
            let mut msg = std::mem::zeroed();
            println!("Running message loop.");
            while winapi::um::winuser::GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
                winapi::um::winuser::TranslateMessage(&msg);
                winapi::um::winuser::DispatchMessageW(&msg);
            }
            println!("Message loop stopped.");
        } else {
            eprintln!("Failed to set the hook.");
        }
    }
}

#[cfg(windows)]
pub fn stop_blocking_chrome() {
    unsafe {
        if let Some(hook) = get_hook_handle() {
            UnhookWindowsHookEx(hook as *mut _);
            println!("Google Chrome blocking stopped.");
            set_hook_handle(null_mut());
        } else {
            eprintln!("No hook is currently set.");
        }
    }
}
