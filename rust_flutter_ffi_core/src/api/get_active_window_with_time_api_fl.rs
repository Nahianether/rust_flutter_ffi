use crate::models::tracforce_get_model::ApiGetModel;
use crate::ActiveWindowWithTime;
use reqwest::Client;
use serde_json::json;
// use std::sync::{Arc, Mutex};

pub async fn send_active_window_data_to_api(
    client: &Client,
    window_data: ActiveWindowWithTime,
    token: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddActiveWindow";
    // let api_url = "https://localhost:7020/api/Tracker/TrackingData";

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let payload = json!({
        "intTrackingTypeId": 7,
        "strTrackingType": "ActiveWindowWithTime",
        "strToken": token,
        "strJsonString": serde_json::to_string(&window_data).unwrap_or_else(|_| "[]".to_string()),
    });

    let response = client
        .post(api_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let api_response: ApiGetModel = response.json().await?;

    if api_response.status_code >= 200 && api_response.status_code < 300 {
        println!("Active window data sent successfully to the API.");
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed with status: {:?}", api_response.message),
        )))
    }
}
