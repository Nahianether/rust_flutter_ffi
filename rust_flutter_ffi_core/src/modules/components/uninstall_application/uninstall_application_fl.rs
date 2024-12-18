#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate winreg;

use std::env;
use std::fs;

use anyhow::Result;
// #[cfg(unix)]
// use tokio::fs::{self};

#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

use crate::modules::components::get_browser_history_database::components::remove_previous_history_fl::remove_previous_history_file;
use crate::modules::components::log_error::log_error_fl::log_error_fn;
use crate::modules::components::uninstall_application::components::delete_executable_fl::delete_executable;

pub async fn uninstall_app_fn() -> Result<(), String> {
    #[cfg(windows)]
    if let Err(e) = remove_startup_registry().await {
        log_error_fn(&format!("Error removing startup registry: {}", e));
    }

    #[cfg(target_os = "macos")]
    if let Err(e) = remove_startup_plist().await {
        log_error_fn(&format!("Error removing startup plist: {}", e));
    }

    #[cfg(target_os = "linux")]
    if let Err(e) = remove_startup_desktop_entry().await {
        log_error_fn(&format!("Error removing startup desktop entry: {}", e));
    }

    if let Err(e) = delete_executable().await {
        log_error_fn(&format!("Error deleting executable: {}", e));
    }

    // remove or delete chrome history file from project during uninstallation
    remove_previous_history_file().await;
    delete_all_screenshot_folder().await;

    println!("Uninstallation completed. Application will exit.");
    std::process::exit(0);
}

#[cfg(windows)]
async fn remove_startup_registry() -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            KEY_WRITE,
        )
        .map_err(|e| format!("Failed to open registry key: {:?}", e))?;

    key.delete_value("track_force_rs")
        .map_err(|e| format!("Failed to delete registry value: {:?}", e))?;

    Ok(())
}

#[cfg(target_os = "macos")]
async fn remove_startup_plist() -> Result<(), String> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let plist_path = home_dir.join("Library/LaunchAgents/track_force_rs.plist");

    if plist_path.exists() {
        fs::remove_file(&plist_path).map_err(|e| format!("Failed to remove plist: {:?}", e))?;
        println!("Removed startup plist: {:?}", plist_path);
    } else {
        println!("Startup plist not found.");
    }

    Ok(())
}

#[cfg(target_os = "linux")]
async fn remove_startup_desktop_entry() -> Result<(), String> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let desktop_path = home_dir.join(".config/autostart/track_force_rs.desktop");

    if desktop_path.exists() {
        fs::remove_file(&desktop_path)
            .await
            .map_err(|e| format!("Failed to remove desktop entry: {:?}", e))?;
        println!("Removed startup desktop entry: {:?}", desktop_path);
    } else {
        println!("Startup desktop entry not found.");
    }

    Ok(())
}

async fn delete_all_screenshot_folder() {
    // delete screenshot folder
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    let screenshot_folder = exe_dir.join("screenshot");
    if screenshot_folder.exists() {
        if let Err(e) = fs::remove_dir_all(&screenshot_folder) {
            log_error_fn(&format!("Error deleting screenshot folder: {}", e));
        } else {
            println!("Screenshot folder deleted successfully.");
        }
    } else {
        println!("Screenshot folder does not exist, skipping deletion.");
    }

    // delete log folder
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    let log_folder = exe_dir.join("log");
    if log_folder.exists() {
        if let Err(e) = fs::remove_dir_all(&log_folder) {
            log_error_fn(&format!("Error deleting log folder: {}", e));
        } else {
            println!("Log folder deleted successfully.");
        }
    } else {
        println!("Log folder does not exist, skipping deletion.");
    }

    // delete screenshot for console folder
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    let screenshot_console_folder = exe_dir.join("screenshot_for_console");
    if screenshot_console_folder.exists() {
        if let Err(e) = fs::remove_dir_all(&screenshot_console_folder) {
            log_error_fn(&format!(
                "Error deleting screenshot for console folder: {}",
                e
            ));
        } else {
            println!("Screenshot for console folder deleted successfully.");
        }
    } else {
        println!("Screenshot for console folder does not exist, skipping deletion.");
    }

    // delete screenshot email folder
    // let exe_path = env::current_exe().expect("Failed to get executable path");
    // let exe_dir = exe_path
    //     .parent()
    //     .expect("Failed to get executable directory");
    // let email_screenshot_folder = exe_dir.join("screenshot_for_email");
    // if email_screenshot_folder.exists() {
    //     if let Err(e) = fs::remove_dir_all(&email_screenshot_folder).await {
    //         log_error_fn(&format!("Error deleting screenshot email folder: {}", e));
    //     } else {
    //         println!("Screenshot folder deleted successfully.");
    //     }
    // } else {
    //     println!("Email Screenshot folder does not exist, skipping deletion.");
    // }

    // delete screenshot meeting folder
    // let exe_path = env::current_exe().expect("Failed to get executable path");
    // let exe_dir = exe_path
    //     .parent()
    //     .expect("Failed to get executable directory");
    // let meeting_screenshot_folder = exe_dir.join("screenshot_for_meeting");
    // if meeting_screenshot_folder.exists() {
    //     if let Err(e) = fs::remove_dir_all(&meeting_screenshot_folder).await {
    //         log_error_fn(&format!("Error deleting screenshot meeting folder: {}", e));
    //     } else {
    //         println!("Screenshot folder deleted successfully.");
    //     }
    // } else {
    //     println!("Meeting Screenshot folder does not exist, skipping deletion.");
    // }

    // delete screenshot chatting folder
    // let exe_path = env::current_exe().expect("Failed to get executable path");
    // let exe_dir = exe_path
    //     .parent()
    //     .expect("Failed to get executable directory");
    // let chatting_screenshot_folder = exe_dir.join("screenshot_for_online_chat");
    // if chatting_screenshot_folder.exists() {
    //     if let Err(e) = fs::remove_dir_all(&chatting_screenshot_folder).await {
    //         log_error_fn(&format!("Error deleting screenshot chatting folder: {}", e));
    //     } else {
    //         println!("Screenshot folder deleted successfully.");
    //     }
    // } else {
    //     println!("Chatting Screenshot folder does not exist, skipping deletion.");
    // }

    // delete screenshot social-media folder
    // let exe_path = env::current_exe().expect("Failed to get executable path");
    // let exe_dir = exe_path
    //     .parent()
    //     .expect("Failed to get executable directory");
    // let social_media_screenshot_folder = exe_dir.join("screenshot_for_social_media");
    // if social_media_screenshot_folder.exists() {
    //     if let Err(e) = fs::remove_dir_all(&social_media_screenshot_folder).await {
    //         log_error_fn(&format!(
    //             "Error deleting screenshot social-media folder: {}",
    //             e
    //         ));
    //     } else {
    //         println!("Screenshot folder deleted successfully.");
    //     }
    // } else {
    //     println!("Social Media Screenshot folder does not exist, skipping deletion.");
    // }
}
