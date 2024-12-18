// #[cfg(windows)]
// extern crate winapi;
// #[cfg(windows)]
// extern crate winreg;

// #[cfg(windows)]
// use winreg::enums::*;
// #[cfg(windows)]
// use winreg::RegKey;

// use crate::log_error::log_error_fl::log_error_fn;

// pub fn set_auto_start_fn() {
//     // Set the application to start automatically on boot
//     let hkcu = RegKey::predef(HKEY_CURRENT_USER);
//     let (key, _disp) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run").unwrap();

//     // Set the path to your executable file for autostart
//     if let Err(e) = key.set_value("track_force_rs", &"D:\\ibos\\track_force\\log\\track_force_rs.exe") {
//         log_error_fn(&format!("Failed to set auto start: {}", e));
//     }
// }

// ---------------------------------------------------------------------------------------------------------------

#[cfg(unix)]
use super::components::set_auto_start_mac_fl::set_auto_start_macos;
#[cfg(windows)]
use super::components::set_auto_start_windows_fl::set_auto_start_windows;

pub async fn set_auto_start_fn() {
    if cfg!(windows) {
        #[cfg(windows)]
        set_auto_start_windows().await;
    } else if cfg!(target_os = "macos") {
        #[cfg(target_os = "macos")]
        set_auto_start_macos().await;
    } else if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        set_auto_start_linux().await;
    } else {
        println!("Unsupported platform for auto-start setup.");
    }
}
