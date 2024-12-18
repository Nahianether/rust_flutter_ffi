use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

use crate::models::{tracforce_get_model::ApiGetModel, tracforce_post_model::ConsoleCapture};

pub async fn send_console_data_to_api_fn(
    client: &Client,
    console_data: &ConsoleCapture,
    token: &str,
) -> Result<()> {
    let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddConsoleCommand";

    let payload = json!({
        "intTrackingTypeId": 16,
        "strTrackingType": "ConsoleCapture",
        "strToken": token,
        "strImage": console_data.image_path,
        "strBody": serde_json::to_string(&console_data.body)?,
    });

    let response = client
        .post(api_url)
        .json(&payload)
        .send()
        .await
        .context("Failed to send request to API")?;

    let api_response: ApiGetModel = response.json().await?;

    if api_response.status_code >= 200 && api_response.status_code < 300 {
        println!("Console Capture data sent to API successfully.");
    } else {
        return Err(anyhow::anyhow!(
            "API responded with status: {}",
            api_response.message
        ));
    }

    Ok(())
}
