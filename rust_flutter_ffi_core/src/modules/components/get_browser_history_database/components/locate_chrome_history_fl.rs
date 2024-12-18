// use std::{env, path::PathBuf};

// pub async fn locate_chrome_history_file() -> Option<PathBuf> {
//     let history_subpath = match env::consts::OS {
//         "windows" => {
//             let local_app_data = env::var("LOCALAPPDATA").ok()?;
//             PathBuf::from(local_app_data).join("Google/Chrome/User Data/Default/History")
//         }
//         "macos" => {
//             let home_dir = dirs::home_dir()?;
//             home_dir.join("Library/Application Support/Google/Chrome/Default/History")
//         }
//         "linux" => {
//             let home_dir = dirs::home_dir()?;
//             home_dir.join(".config/google-chrome/Default/History")
//         }
//         _ => return None,
//     };

//     if history_subpath.exists() {
//         Some(history_subpath)
//     } else {
//         None
//     }
// }

#[cfg(unix)]
use std::path::Path;
use std::path::PathBuf;

use crate::modules::components::log_error::log_error_fl::log_error_fn;

pub async fn locate_chrome_history_file() -> Option<PathBuf> {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            locate_chrome_history_windows().await
        } else if #[cfg(target_os = "macos")] {
            locate_chrome_history_macos().await
        } else if #[cfg(target_os = "linux")] {
            locate_chrome_history_linux().await
        } else {
            None
        }
    }
}

#[cfg(windows)]
async fn locate_chrome_history_windows() -> Option<PathBuf> {
    if let Some(local_app_data) = env::var("LOCALAPPDATA").ok() {
        let default_path =
            PathBuf::from(local_app_data).join("Google/Chrome/User Data/Default/History");
        if default_path.exists() {
            return Some(default_path);
        }
    }

    use std::env;

    use winreg::enums::*;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(chrome_key) = hklm.open_subkey("Software\\Google\\Chrome\\BLBeacon") {
        if let Ok(chrome_install_path) = chrome_key.get_value::<String, _>("path") {
            let custom_path = PathBuf::from(chrome_install_path).join("User Data/Default/History");
            if custom_path.exists() {
                return Some(custom_path);
            }
        }
    }

    log_error_fn("Failed to locate Chrome history file on Windows.");
    None
}

#[cfg(target_os = "macos")]
async fn locate_chrome_history_macos() -> Option<PathBuf> {
    if let Some(home_dir) = dirs::home_dir() {
        let default_path =
            home_dir.join("Library/Application Support/Google/Chrome/Default/History");
        if default_path.exists() {
            return Some(default_path);
        }
    }

    let possible_dirs = vec![
        "/Applications/Google Chrome.app",
        "~/Applications/Google Chrome.app",
    ];

    for dir in possible_dirs {
        let path = Path::new(dir).join("Default/History");
        if path.exists() {
            return Some(path);
        }
    }

    log_error_fn("Failed to locate Chrome history file on macOS.");
    None
}

#[cfg(target_os = "linux")]
async fn locate_chrome_history_linux() -> Option<PathBuf> {
    if let Some(home_dir) = dirs::home_dir() {
        let default_path = home_dir.join(".config/google-chrome/Default/History");
        if default_path.exists() {
            return Some(default_path);
        }

        let chromium_path = home_dir.join(".config/chromium/Default/History");
        if chromium_path.exists() {
            return Some(chromium_path);
        }
    }

    log_error_fn("Failed to locate Chrome history file on Linux.");
    None
}
