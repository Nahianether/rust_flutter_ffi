// #[cfg(unix)]
// use anyhow::{Context, Result};
// #[cfg(unix)]
// use reqwest::Client;
// #[cfg(unix)]
// use serde_json::json;

// #[cfg(unix)]
// use crate::models::tracforce_post_model::OnlineMessageDetect;

// #[cfg(unix)]
// pub async fn send_detect_chat_to_api_fn(
//     client: &Client,
//     chatting: &OnlineMessageDetect,
//     token: &str,
// ) -> Result<()> {
//     use crate::modules::components::log_error::log_error_fl::log_error_fn;

//     let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddChatHistory";

//     let payload = json!({
//         "intTrackingTypeId": 15,
//         "strTrackingType": "ChatImage",
//         "strToken": token,
//         "strImage": chatting.image_path,
//         "strChatting": serde_json::to_string(&chatting.message)?,
//     });

//     let response = client
//         .post(api_url)
//         .json(&payload)
//         .send()
//         .await
//         .context("Failed to send request to API")?;

//     if response.status().is_success() {
//         println!("Chat Image sent to API successfully.");
//     } else {
//         let status = response.status();
//         let body = response
//             .text()
//             .await
//             .unwrap_or_else(|_| "Failed to read response body".to_string());
//         log_error_fn(&format!(
//             "Failed to send Chat image. Status: {}, Body: {}",
//             status, body
//         ));
//         return Err(anyhow::anyhow!("API responded with status: {}", status));
//     }

//     Ok(())
// }
