use crate::modules::components::log_error::log_error_fl::log_error_fn;
use tokio::time::{sleep, Duration};

use super::components::is_app_installed_by_file_fl::is_app_installed_by_files;
use super::components::is_app_running_fl::is_app_running;

const RETRY_ATTEMPTS: u8 = 3;
const RETRY_DELAY: Duration = Duration::from_secs(2);

pub async fn is_app_installed_completely_fn() -> bool {
    let installed_by_registry = is_app_installed().await;
    let installed_by_files = is_app_installed_by_files().await;
    let app_running = is_app_running().await;

    installed_by_registry && installed_by_files && !app_running
}

#[cfg(windows)]
async fn is_app_installed() -> bool {
    use tokio::task;
    use winreg::enums::*;
    use winreg::RegKey;

    #[cfg(not(debug_assertions))]
    use crate::components::get_employee_id_from_app_name_fl::get_employee_id_fn;

    for attempt in 1..=RETRY_ATTEMPTS {
        let result = task::spawn_blocking(move || {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let key = hkcu.open_subkey_with_flags(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                KEY_READ,
            );
            #[cfg(not(debug_assertions))]
            let employee_id = get_employee_id_fn();
            #[cfg(debug_assertions)]
            let employee_id = "fee5cecd-3472-42e4-b02b-6b7aa50f1ff9".to_string();
            let app_name = format!("{}_track_force_rs", employee_id);
            // let app_name = "track_force_rs".to_string();
            match key {
                Ok(key) => key.get_value::<String, _>(app_name).is_ok(),
                Err(_) => false,
            }
        })
        .await
        .unwrap_or(false);

        if result {
            println!("App is installed.");
            return true;
        } else {
            log_error_fn(&format!("Attempt {}: App is not installed.", attempt));
            if attempt < RETRY_ATTEMPTS {
                sleep(RETRY_DELAY).await;
            }
        }
    }
    false
}

#[cfg(target_os = "macos")]
async fn is_app_installed() -> bool {
    let plist_path = dirs::home_dir()
        .unwrap()
        .join("Library/LaunchAgents/track_force_rs.plist");

    for attempt in 1..=RETRY_ATTEMPTS {
        if tokio::fs::metadata(&plist_path).await.is_ok() {
            println!("App is installed (macOS - plist).");
            return true;
        } else {
            log_error_fn(&format!(
                "Attempt {}: App is not installed (macOS - plist).",
                attempt
            ));
            if attempt < RETRY_ATTEMPTS {
                sleep(RETRY_DELAY).await;
            }
        }
    }
    false
}

#[cfg(target_os = "linux")]
async fn is_app_installed() -> bool {
    let desktop_path = dirs::home_dir()
        .unwrap()
        .join(".config/autostart/track_force_rs.desktop");

    for attempt in 1..=RETRY_ATTEMPTS {
        if tokio::fs::metadata(&desktop_path).await.is_ok() {
            println!("App is installed (Linux - desktop entry).");
            return true;
        } else {
            log_error_fn(&format!(
                "Attempt {}: App is not installed (Linux - desktop entry).",
                attempt
            ));
            if attempt < RETRY_ATTEMPTS {
                sleep(RETRY_DELAY).await;
            }
        }
    }
    false
}
