use reqwest::Client;
use std::error::Error;
use std::time::Duration;

use crate::api::send_running_process_api_fl::send_processes_to_api;
use crate::modules::components::get_running_process_list::components::get_running_process_fl::get_running_processes_fn;
use crate::modules::components::log_error::log_error_fl::log_error_fn;

// Asynchronous function to periodically check running processes and send them to the API
pub async fn trigger_process_check_every_5m_fn(
    x: i32,
    token: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    loop {
        println!("Checking running processes...");

        // Step 1: Fetch running processes asynchronously
        let processes = get_running_processes_fn().await;

        // Step 2: Send running processes to API
        if let Err(e) = send_processes_to_api(&client, &processes, &token).await {
            log_error_fn(&format!("Failed to send running processes to API: {:?}", e));
        } else {
            println!("Running processes sent to API.");
        }

        // Step 3: Sleep asynchronously for the specified interval
        tokio::time::sleep(Duration::from_secs(x as u64)).await;
    }
}
