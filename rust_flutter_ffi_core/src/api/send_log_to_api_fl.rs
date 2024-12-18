use reqwest::Client;
use serde_json::json;
use std::error::Error;

use crate::models::{tracforce_get_model::ApiGetModel, tracforce_post_model::ErrorLog};

pub async fn send_log_to_api_fn(
    client: &Client,
    token: &str,
    log_contents: ErrorLog,
) -> Result<(), Box<dyn Error>> {
    let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddErrorLog";

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let serialized_logs = serde_json::to_string(&log_contents)
        .map_err(|e| format!("Failed to serialize log contents: {:?}", e))?;

    let payload = json!({
        "intTrackingTypeId": 10,
        "strTrackingType": "ErrorLog",
        "strToken": token,
        "strJsonString": serialized_logs,
    });

    let response = client
        .post(api_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let api_response: ApiGetModel = response.json().await?;

    if api_response.status_code >= 200 && api_response.status_code < 300 {
        println!("Log data sent successfully to the API.");
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed with status: {:?}", api_response.message),
        )))
    }
}
