use crate::api::send_desktop_details_api_fl::send_desktop_details_to_api;
use crate::modules::components::desktop_details_info::components::fetch_desktop_details_fl::fetch_desktop_details;
use crate::modules::components::log_error::log_error_fl::log_error_fn;
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

pub async fn get_desktop_details_fn(token: String) -> Result<()> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    let mut attempts = 1;
    let desktop_details = loop {
        match fetch_desktop_details().await {
            Ok(details) => break details,
            Err(e) => {
                log_error_fn(&format!(
                    "Failed to get desktop details (attempt {}): {:?}",
                    attempts, e
                ));
                if attempts >= 3 {
                    return Err(e);
                }
                attempts += 1;
                sleep(Duration::from_secs(30)).await;
            }
        }
    };

    for attempt in 1..=3 {
        match send_desktop_details_to_api(&client, &desktop_details, &token).await {
            Ok(_) => {
                println!("Desktop details sent to API.");
                return Ok(());
            }
            Err(e) => {
                println!(
                    "Failed to send desktop details to API (attempt {}): {:?}",
                    attempt, e
                );
                if attempt < 3 {
                    sleep(Duration::from_secs(30)).await;
                } else {
                    log_error_fn(&format!("Failed to send desktop details to API: {:?}", e));
                    return Err(e);
                }
            }
        }
    }

    println!("Desktop details sent to API.");

    Ok(())
}
