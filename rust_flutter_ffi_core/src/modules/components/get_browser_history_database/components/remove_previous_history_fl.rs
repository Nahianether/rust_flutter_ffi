use crate::modules::components::log_error::log_error_fl::log_error_fn;
use std::env;
use tokio::fs as async_fs;

pub async fn remove_previous_history_file() {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    let previous_file_path = exe_dir.join("chrome_history.db");

    if previous_file_path.exists() {
        if let Err(e) = async_fs::remove_file(&previous_file_path).await {
            println!("Failed to remove previous history file.");
            log_error_fn(&format!("Failed to remove previous history file: {:?}", e));
        } else {
            println!("Previous history file removed.");
        }
    }
}
