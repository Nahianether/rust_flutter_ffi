// #[cfg(unix)]
// use anyhow::{Context, Result};
// #[cfg(unix)]
// use reqwest::Client;
// #[cfg(unix)]
// use serde_json::json;

// #[cfg(unix)]
// use crate::models::tracforce_post_model::SocialMediaDetect;

// #[cfg(unix)]
// pub async fn send_detect_social_media_to_api_fn(
//     client: &Client,
//     social_media: &SocialMediaDetect,
//     token: &str,
// ) -> Result<()> {
//     use crate::modules::components::log_error::log_error_fl::log_error_fn;

//     let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddSocialMediaDetails";

//     let payload = json!({
//         "intTrackingTypeId": 14,
//         "strTrackingType": "SocialMediaImage",
//         "strToken": token,
//         "strImage": serde_json::to_string(&social_media.image_path)?,
//         "strSocialMedia": social_media.social_media,
//     });

//     let response = client
//         .post(api_url)
//         .json(&payload)
//         .send()
//         .await
//         .context("Failed to send request to API")?;

//     if response.status().is_success() {
//         println!("Social Media Image sent to API successfully.");
//     } else {
//         let status = response.status();
//         let body = response
//             .text()
//             .await
//             .unwrap_or_else(|_| "Failed to read response body".to_string());
//         log_error_fn(&format!(
//             "Failed to send Social Media image. Status: {}, Body: {}",
//             status, body
//         ));
//         return Err(anyhow::anyhow!("API responded with status: {}", status));
//     }

//     Ok(())
// }
