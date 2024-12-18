// use anyhow::{Context, Result};
// use reqwest::Client;
// use serde_json::json;

// use crate::models::tracforce_post_model::EmailDetails;

// pub async fn send_detect_email_to_api_fn(
//     client: &Client,
//     text_data: &EmailDetails,
//     token: &str,
// ) -> Result<()> {
//     use crate::modules::components::log_error::log_error_fl::log_error_fn;

//     let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddEmailDetails";

//     let payload = json!({
//         "intTrackingTypeId": 11,
//         "strTrackingType": "EmailFromImage",
//         "strToken": token,
//         "strImage": text_data.image_path,
//         "strBody": serde_json::to_string(&text_data.body)?,
//     });

//     let response = client
//         .post(api_url)
//         .json(&payload)
//         .send()
//         .await
//         .context("Failed to send request to API")?;

//     if response.status().is_success() {
//         println!("OCR email data sent to API successfully.");
//     } else {
//         let status = response.status();
//         let body = response
//             .text()
//             .await
//             .unwrap_or_else(|_| "Failed to read response body".to_string());
//         log_error_fn(&format!(
//             "Failed to send OCR email data. Status: {}, Body: {}",
//             status, body
//         ));
//         return Err(anyhow::anyhow!("API responded with status: {}", status));
//     }

//     Ok(())
// }
