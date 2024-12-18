use crate::{models::tracforce_get_model::ApiGetModel, BrowserHistory};
use reqwest::Client;
use serde_json::json;
// use std::sync::{Arc, Mutex};

pub async fn send_browser_history_to_api(
    client: &Client,
    browser_history: BrowserHistory,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddWebHistory";

    println!("Sending browser history to the API...");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let payload = json!({
        "intTrackingTypeId": 8,
        "strTrackingType": "BrowserHistory",
        "strToken": token,
        "strJsonString": serde_json::to_string(&browser_history).unwrap_or_else(|_| "[]".to_string()),
    });

    let response = client
        .post(api_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let api_response: ApiGetModel = response.json().await?;

    if api_response.status_code >= 200 && api_response.status_code < 300 {
        println!("Browser history sent successfully to the API.");
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed with status: {:?}", api_response.message),
        )))
    }
}
