// use crate::log_error::log_error_fl::log_error_fn;

// pub fn get_chrome_history_fn(path: &str) -> Vec<String> {
//     let query =
//         "SELECT url, title, last_visit_time FROM urls ORDER BY last_visit_time DESC LIMIT 10";

//     let conn = match rusqlite::Connection::open(path) {
//         Ok(connection) => connection,
//         Err(e) => {
//             println!("Failed to open SQLite connection");
//             log_error_fn(&format!("Failed to open SQLite connection: {:?}", e));
//             return Vec::new();
//         }
//     };

//     let mut stmt = match conn.prepare(query) {
//         Ok(statement) => statement,
//         Err(e) => {
//             println!("Failed to prepare SQL query");
//             log_error_fn(&format!("Failed to prepare SQL query: {:?}", e));
//             return Vec::new();
//         }
//     };

//     let rows = match stmt.query_map([], |row| {
//         Ok(format!(
//             "{:?}: {:?} ({:?})",
//             row.get::<usize, String>(0),
//             row.get::<usize, String>(1),
//             row.get::<usize, i64>(2)
//         ))
//     }) {
//         Ok(rows) => rows,
//         Err(e) => {
//             println!("Failed to execute SQL query");
//             log_error_fn(&format!("Failed to execute SQL query: {:?}", e));
//             return Vec::new();
//         }
//     };

//     let mut history = Vec::new();

//     for row in rows {
//         match row {
//             Ok(row_data) => {
//                 println!("{}", row_data);
//                 history.push(row_data);
//             }
//             Err(e) => {
//                 println!("Failed to parse row");
//                 log_error_fn(&format!("Failed to parse row: {:?}", e));
//             }
//         }
//     }

//     history
// }

// ---------------------------------------------------------------------------------------------

use crate::models::tracforce_post_model::BrowserHistorySingle;
use crate::modules::components::log_error::log_error_fl::log_error_fn;
use crate::{api::send_browser_history_api_fl::send_browser_history_to_api, BrowserHistory};
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

use super::components::remove_previous_history_fl::remove_previous_history_file;
use super::components::{
    copy_history_file_to_project_fl::copy_history_file_to_project,
    locate_chrome_history_fl::locate_chrome_history_file,
    read_chrome_history_from_db_fl::read_chrome_history_from_db,
};

pub async fn get_chrome_history_fn(x: u64, token: String) -> Result<()> {
    // Remove previous history file once at startup
    remove_previous_history_file().await;

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    loop {
        let mut history_buffer: Vec<BrowserHistorySingle> = Vec::new();

        if let Some(history_path) = locate_chrome_history_file().await {
            let project_history_path = copy_history_file_to_project(&history_path).await;
            let history_entries = read_chrome_history_from_db(&project_history_path).await;

            history_buffer.extend(history_entries);

            if !history_buffer.is_empty() {
                let payload = BrowserHistory {
                    url: history_buffer.clone(),
                };

                if let Err(e) = send_browser_history_to_api(&client, payload, &token).await {
                    log_error_fn(&format!("Failed to send browser history to API: {:?}", e));
                } else {
                    println!("Browser history data sent to API.");
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
