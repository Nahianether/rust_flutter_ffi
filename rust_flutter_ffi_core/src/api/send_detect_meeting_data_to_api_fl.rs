// use anyhow::{Context, Result};
// use reqwest::Client;
// use serde_json::json;

// use crate::models::tracforce_post_model::MeetingDetails;

// pub async fn send_detect_meeting_data_to_api_fn(
//     client: &Client,
//     text_data: &MeetingDetails,
//     token: &str,
// ) -> Result<()> {
//     use crate::modules::components::log_error::log_error_fl::log_error_fn;

//     let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddMettingDetails";

//     let payload = json!({
//         "intTrackingTypeId": 12,
//         "strTrackingType": "MeetingFromImage",
//         "strToken": token,
//         "strImage": text_data.image_path,
//         "strBody": serde_json::to_string(&text_data.participant)?,
//     });

//     let response = client
//         .post(api_url)
//         .json(&payload)
//         .send()
//         .await
//         .context("Failed to send request to API")?;

//     if response.status().is_success() {
//         println!("OCR Meeting data sent to API successfully.");
//     } else {
//         let status = response.status();
//         let body = response
//             .text()
//             .await
//             .unwrap_or_else(|_| "Failed to read response body".to_string());
//         log_error_fn(&format!(
//             "Failed to send OCR meeting data. Status: {}, Body: {}",
//             status, body
//         ));
//         return Err(anyhow::anyhow!("API responded with status: {}", status));
//     }

//     Ok(())
// }
