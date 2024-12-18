#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate winreg;

use crate::modules::components::log_error::log_error_fl::log_error_fn;
#[cfg(unix)]
use tokio::process::Command;

pub async fn hide_window_fn() {
    #[cfg(windows)]
    hide_window_windows().await;

    #[cfg(target_os = "macos")]
    hide_window_macos().await;

    #[cfg(target_os = "linux")]
    hide_window_linux().await;
}

#[cfg(windows)]
async fn hide_window_windows() {
    use tokio::task;
    use winapi::um::consoleapi::AllocConsole;
    use winapi::um::wincon::{FreeConsole, GetConsoleWindow};
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    // Run the Windows console operations in a blocking thread to avoid blocking async runtime
    task::spawn_blocking(|| {
        unsafe {
            // Try to allocate a console if none exists
            AllocConsole();

            // Now retrieve the console window handle
            let console_window = GetConsoleWindow();
            if !console_window.is_null() {
                // Hide the console window
                ShowWindow(console_window, SW_HIDE);
                FreeConsole(); // Optional: detach the console if you don’t need it further
            } else {
                log_error_fn("Failed to find or allocate the console window to hide on Windows.");
            }
        }
    })
    .await
    .unwrap_or_else(|e| {
        log_error_fn(&format!(
            "Failed to run blocking task for Windows console: {:?}",
            e
        ));
    });
}

#[cfg(target_os = "macos")]
async fn hide_window_macos() {
    let script = r#"
        tell application "Terminal"
            set miniaturized of front window to true
        end tell
    "#;

    if let Err(e) = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .await
    {
        log_error_fn(&format!("Failed to hide window on macOS: {}", e));
    }
}

#[cfg(target_os = "linux")]
async fn hide_window_linux() {
    if let Err(e) = Command::new("xdotool")
        .arg("getactivewindow")
        .arg("windowminimize")
        .output()
        .await
    {
        log_error_fn(&format!("Failed to hide window on Linux: {}", e));
    }
}
