#[cfg(unix)]
use crate::modules::components::log_error::log_error_fl::log_error_fn;
#[cfg(unix)]
use tokio::time::{sleep, Duration};

#[cfg(unix)]
pub async fn set_auto_start_macos() {
    use dirs::home_dir;
    use std::env;
    use tokio::fs;

    let exe_path = env::current_exe().unwrap().display().to_string();
    let plist_content = format!(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>track_force_rs</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
"#,
        exe_path
    );

    let plist_path = home_dir()
        .unwrap()
        .join("Library/LaunchAgents")
        .join("track_force_rs.plist");

    for attempt in 1..=3 {
        if fs::write(&plist_path, &plist_content).await.is_ok() {
            println!("Auto-start successfully set for macOS.");
            return;
        }

        println!(
            "Attempt {} to set auto-start failed on macOS. Retrying...",
            attempt
        );
        log_error_fn(&format!(
            "Attempt {} to set auto-start failed on macOS.",
            attempt
        ));

        if attempt < 3 {
            sleep(Duration::from_secs(2)).await;
        } else {
            println!("Failed to set auto-start on macOS after 3 attempts.");
            log_error_fn("Failed to set auto-start on macOS after 3 attempts.");
        }
    }
}
