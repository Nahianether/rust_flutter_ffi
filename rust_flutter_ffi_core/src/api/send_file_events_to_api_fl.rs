use crate::{
    models::{tracforce_get_model::ApiGetModel, tracforce_post_model::FileMonitoringPayload},
    modules::components::log_error::log_error_fl::log_error_fn,
};
use anyhow::Result;
use reqwest::Client;
use serde_json::json;

pub async fn send_file_events_to_api(
    client: &Client,
    events: &FileMonitoringPayload,
    token: &String,
) -> Result<()> {
    let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddFileEvent";

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let payload = json!({
        "intTrackingTypeId": 2,
        "strTrackingType": "FileSystemEvents",
        "strToken": token,
        "strJsonString": serde_json::to_string(events).unwrap_or_else(|_| "[]".to_string()),
    });

    let response = client.post(api_url).json(&payload).send().await?;

    let api_response: ApiGetModel = response.json().await?;

    if api_response.status_code >= 200 && api_response.status_code < 300 {
        println!("File system monitoring data sent to API successfully.");
    } else {
        log_error_fn(&format!(
            "Failed to send File system monitoring data. Status: {}, Message: {}",
            api_response.status_code, api_response.message
        ));
        return Err(anyhow::anyhow!(
            "API responded with status: {}",
            api_response.message
        ));
    }

    Ok(())
}
