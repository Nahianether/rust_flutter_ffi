use crate::models::{tracforce_get_model::ApiGetModel, tracforce_post_model::SearchKeyFromUrlList};
use reqwest::Client;
use serde_json::json;
// use std::sync::{Arc, Mutex};

pub async fn send_search_key_from_url_api_fn(
    client: &Client,
    search_key_from_url_list: &SearchKeyFromUrlList,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = "https://app.trackforce.io/api/TrackerDesktop/AddSearchKey";

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let payload = json!({
        "intTrackingTypeId": 13,
        "strTrackingType": "SearchKeyFromUrl",
        "strToken": token,
        "strJsonString": serde_json::to_string(search_key_from_url_list).unwrap(),
    });

    let response = client
        .post(api_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let api_response: ApiGetModel = response.json().await?;

    if api_response.status_code >= 200 && api_response.status_code < 300 {
        println!("Search Key from URL sent successfully to the API.");
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed with status: {:?}", api_response.message),
        )))
    }
}
