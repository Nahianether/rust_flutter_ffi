use std::time::Duration;

use crate::api::search_key_from_url_api_fl::send_search_key_from_url_api_fn;
use crate::models::tracforce_post_model::{SearchKeyFromUrl, SearchKeyFromUrlList};
use crate::modules::components::get_browser_history_database::components::copy_history_file_to_project_fl::copy_history_file_to_project;
use crate::modules::components::get_browser_history_database::components::locate_chrome_history_fl::locate_chrome_history_file;
use crate::modules::components::get_browser_history_database::components::remove_previous_history_fl::remove_previous_history_file;
use crate::modules::components::log_error::log_error_fl::log_error_fn;
use anyhow::Result;
use reqwest::Client;
use tokio::time::sleep;

use super::components::read_chrome_history_for_search_key_from_db_fl::read_chrome_history_for_search_key_from_db_fn;

pub async fn get_search_key_from_url_fn(x: u64, token: String) -> Result<()> {
    remove_previous_history_file().await;

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    loop {
        let mut history_buffer: Vec<SearchKeyFromUrl> = Vec::new();

        if let Some(history_path) = locate_chrome_history_file().await {
            let project_history_path = copy_history_file_to_project(&history_path).await;
            let history_entries =
                read_chrome_history_for_search_key_from_db_fn(&project_history_path).await;

            history_buffer.extend(history_entries);

            if !history_buffer.is_empty() {
                let payload = SearchKeyFromUrlList {
                    search_key_from_url: history_buffer.clone(),
                };

                if let Err(e) = send_search_key_from_url_api_fn(&client, &payload, &token).await {
                    log_error_fn(&format!(
                        "Failed to send search key from url to API: {:?}",
                        e
                    ));
                    println!("Failed to send search key from url to API: {:?}", e);
                } else {
                    println!("Search key from URL data sent to API.");
                    history_buffer.clear();
                }
            } else {
                println!("No new browser history entries found.");
            }
        } else {
            log_error_fn("Failed to locate Chrome history file.");
        }

        sleep(Duration::from_secs(x as u64)).await;
    }
}
