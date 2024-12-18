use crate::modules::components::log_error::log_error_fl::log_error_fn;
use std::{
    env,
    path::{Path, PathBuf},
};
use tokio::fs as async_fs;

pub async fn copy_history_file_to_project(history_path: &Path) -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    let destination_path = exe_dir.join("chrome_history.db");

    if async_fs::copy(history_path, &destination_path)
        .await
        .is_ok()
    {
        println!("Successfully copied history file to project folder.");
    } else {
        println!("Failed to copy history file.");
        log_error_fn("Failed to copy history file.");
    }

    destination_path
}
