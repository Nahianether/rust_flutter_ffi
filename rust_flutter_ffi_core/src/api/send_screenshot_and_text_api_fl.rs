use anyhow::Result;
use reqwest::Client;
use serde_json::json;
// use std::sync::{Arc, Mutex};

use crate::models::tracforce_get_model::ApiGetModel;
use crate::models::tracforce_post_model::TextFromImage;
use crate::modules::components::log_error::log_error_fl::log_error_fn;
// use crate::{DESKTOP_DETAILS_DATA, DESKTOP_DETAILS_ID};

pub async fn send_text_to_api(
    client: &Client,
    text_data: &TextFromImage,
    token: &str,
) -> Result<()> {
    use crate::modules::components::log_error::log_error_fl::log_error_fn;

    let api_url = "https://app.trackforce.io/api/Tracker/TrackingData";
    // let api_url = "https://localhost:7020/api/Tracker/TrackingData";

    let payload = json!({
        "intTrackingTypeId": 4,
        "strTrackingType": "TextFromImage",
        "strToken": token,
        "strJsonString": serde_json::to_string(&text_data).unwrap_or_else(|_| "[]".to_string()),
    });
    let response = client.post(api_url).json(&payload).send().await?;
    if response.status().is_success() {
        println!("OCR data sent to API.");
    } else {
        log_error_fn("Failed to send OCR data to API");
    }
    Ok(())
}

pub async fn send_screenshot_to_api(
    client: &Client,
    screenshot_data: &String,
    token: &str,
) -> Result<()> {
    let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddImageInfo";
    // let api_url = "https://localhost:7020/api/Tracker/TrackingData";

    let payload = json!({
        "intTrackingTypeId": 3,
        "strTrackingType": "Screenshot",
        "strToken": token,
        "strJsonString": &screenshot_data,
        // serde_json::to_string(&console_data.body)?,
    });
    let response = client.post(api_url).json(&payload).send().await?;

    let api_response: ApiGetModel = response.json().await?;

    if api_response.status_code >= 200 && api_response.status_code < 300 {
        println!("Screenshot data sent to API.");
    } else {
        println!(
            "Failed to send screenshot data to API with status: {:?}",
            api_response.message
        );
        log_error_fn(
            format!(
                "Failed to send screenshot to API: {:?}",
                api_response.message
            )
            .as_str(),
        );
    }
    Ok(())
}
