use crate::{models::tracforce_get_model::ApiGetModel, MouseKeyboardMovement};
use reqwest::Client;
use serde_json::json;
// use std::sync::{Arc, Mutex};

pub async fn send_event_data_to_api(
    client: &Client,
    data: &MouseKeyboardMovement,
    token: &String,
) -> anyhow::Result<()> {
    let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddMouseKeyBoardEvent";

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let payload = json!({
        "intTrackingTypeId": 5,
        "strTrackingType": "MouseKeyboardEvent",
        "strToken": token,
        "strJsonString": serde_json::to_string(&data).unwrap_or_else(|_| "[]".to_string()),
    });

    let response = client
        .post(api_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let api_response: ApiGetModel = response.json().await?;

    if api_response.status_code >= 200 && api_response.status_code < 300 {
        println!("Event data sent successfully to the API.");
        Ok(())
    } else {
        Err(anyhow::Error::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed with status: {:?}", api_response.message),
        )))
    }
}
