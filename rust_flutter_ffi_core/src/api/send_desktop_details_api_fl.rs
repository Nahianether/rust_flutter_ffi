use crate::{models::tracforce_get_model::ApiGetModel, DesktopDetails};
use anyhow::{Error, Result};
use reqwest::Client;
use serde_json::json;

pub async fn send_desktop_details_to_api(
    client: &Client,
    desktop_details: &DesktopDetails,
    token: &String,
) -> Result<(), Error> {
    let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddDesktopDetails";

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let payload = json!({
        "intTrackingTypeId": 1,
        "strTrackingType": "DesktopDetails",
        "strToken": token,
        "strJsonString": serde_json::to_string(&desktop_details).unwrap_or_else(|_| "[]".to_string()),
    });

    let response = client
        .post(api_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let api_response: ApiGetModel = response.json().await?;

    if api_response.status_code >= 200 && api_response.status_code < 300 {
        println!("Desktop Details sent successfully to the API.");
        Ok(())
    } else {
        Err(Error::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed with status: {:?}", api_response.message),
        )))
    }
}
