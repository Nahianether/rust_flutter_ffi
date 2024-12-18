#[cfg(target_os = "linux")]
pub async fn set_auto_start_linux() {
    use dirs::home_dir;
    use std::env;
    use tokio::fs;

    let exe_path = env::current_exe().unwrap().display().to_string();
    let desktop_content = format!(
        "[Desktop Entry]
Name=track_force_rs
Exec={}
Type=Application
X-GNOME-Autostart-enabled=true
",
        exe_path
    );

    let autostart_dir = home_dir()
        .unwrap()
        .join(".config/autostart")
        .join("track_force_rs.desktop");

    for attempt in 1..=3 {
        if fs::create_dir_all(autostart_dir.parent().unwrap())
            .await
            .is_ok()
            && fs::write(&autostart_dir, &desktop_content).await.is_ok()
        {
            println!("Auto-start successfully set for Linux.");
            return;
        }

        println!(
            "Attempt {} to set auto-start failed on Linux. Retrying...",
            attempt
        );
        log_error_fn(&format!(
            "Attempt {} to set auto-start failed on Linux.",
            attempt
        ));

        if attempt < 3 {
            sleep(Duration::from_secs(2)).await;
        } else {
            println!("Failed to set auto-start on Linux after 3 attempts.");
            log_error_fn("Failed to set auto-start on Linux after 3 attempts.");
        }
    }
}
