use reqwest::Client;
use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::api::send_log_to_api_fl::send_log_to_api_fn;
use crate::models::tracforce_post_model::{ErrorLog, ErrorLogSingle};
use crate::modules::components::log_error::log_error_fl::log_error_fn;

pub async fn send_log_to_api(token: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    let exe_path = env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    let log_dir_path = exe_dir.join("log");
    let log_file_path = log_dir_path.join("track_force_rs.log");

    if !log_dir_path.exists() || !log_file_path.exists() {
        println!("Log directory or log file does not exist. Skipping API call.");
        return Ok(());
    }

    let mut file = match File::open(&log_file_path).await {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open log file: {:?}", e);
            return Ok(());
        }
    };

    let mut log_contents = String::new();
    file.read_to_string(&mut log_contents).await?;

    if log_contents.is_empty() {
        println!("Log file is empty. Skipping API call.");
        return Ok(());
    }

    let error_logs: Vec<ErrorLogSingle> = log_contents
        .lines()
        .map(|line| ErrorLogSingle {
            error_log: line.to_string(),
        })
        .collect();

    let error_log_model = ErrorLog { error_logs };

    match send_log_to_api_fn(&client, token, error_log_model).await {
        Ok(_) => {
            println!("Log data sent successfully to the API.");
            // Remove the log file after sending the data
            std::fs::remove_file(&log_file_path)?;
            Ok(())
        }
        Err(e) => {
            println!("Failed to send log data to the API: {:?}", e);
            log_error_fn(&format!("Failed to send log data to the API: {:?}", e));
            Err(e)
        }
    }
}
