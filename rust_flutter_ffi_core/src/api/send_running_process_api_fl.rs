use reqwest::Client;
use serde_json::json;
use std::error::Error;
// use std::sync::{Arc, Mutex};

use crate::models::tracforce_post_model::RunningProcess;
// use crate::{DESKTOP_DETAILS_DATA, DESKTOP_DETAILS_ID};

pub async fn send_processes_to_api(
    client: &Client,
    processes: &[RunningProcess],
    token: &String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let api_url = "https://app.trackforce.io/api/Tracker/TrackingData";
    // let api_url = "https://localhost:7020/api/Tracker/TrackingData";

    // let desktop_details_id = Arc::new(Mutex::new(DESKTOP_DETAILS_ID.lock().unwrap().clone()));
    // let desktop_details_name = Arc::new(Mutex::new(DESKTOP_DETAILS_DATA.lock().unwrap().clone()));

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let payload = json!({
        "intTrackingTypeId": 6,
        "strTrackingType": "RunningProcess",
        "strToken": token,
        "strJsonString": serde_json::to_string(&processes).unwrap_or_else(|_| "[]".to_string()),
        // "desktopId": desktop_details_id.lock().unwrap().clone(),
        // "desktopName": desktop_details_name.lock().unwrap().clone(),
    });

    let response = client
        .post(api_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            println!("Running Processes sent successfully to the API.");
            Ok(())
        }
        Ok(resp) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed with status: {:?}", resp.status()),
        ))),
        Err(e) => {
            println!("Failed to send Running Process to the API: {:?}", e);
            Err(Box::new(e))
        }
    }
}
