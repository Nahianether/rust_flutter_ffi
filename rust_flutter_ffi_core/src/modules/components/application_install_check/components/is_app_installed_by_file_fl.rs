use crate::modules::components::log_error::log_error_fl::log_error_fn;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

const RETRY_ATTEMPTS: u8 = 3;
const RETRY_DELAY: Duration = Duration::from_secs(2);

pub async fn is_app_installed_by_files() -> bool {
    for attempt in 1..=RETRY_ATTEMPTS {
        match tokio::fs::canonicalize(std::env::current_exe().unwrap_or(PathBuf::new())).await {
            Ok(exe_path) => {
                if exe_path.exists() {
                    println!("App executable exists.");
                    return true;
                } else {
                    log_error_fn(&format!("Attempt {}: App executable not found.", attempt));
                }
            }
            Err(e) => {
                log_error_fn(&format!(
                    "Attempt {}: Failed to get current executable path: {:?}",
                    attempt, e
                ));
            }
        }
        if attempt < RETRY_ATTEMPTS {
            sleep(RETRY_DELAY).await;
        }
    }
    false
}
