use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Result;
use reqwest::Client;

use crate::{models::tracforce_post_model::ConfigFile, DESKTOP_MACADD_DATA, SHARED_PAYLOAD};

pub async fn fetch_configuration_from_api() -> Result<ConfigFile> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");
    let mac_address_data = Arc::new(Mutex::new(DESKTOP_MACADD_DATA.lock().unwrap().clone()));
    let shared_payload = Arc::new(Mutex::new(SHARED_PAYLOAD.lock().unwrap().clone()));

    let token = shared_payload.lock().unwrap().clone();
    let mac = mac_address_data.lock().unwrap().clone();

    let api_url = format!("https://app.trackforce.io/api/TrackerDesktop/GetRestrictionByEmployeeToken?token={}&macAddress={}", token, mac);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let response = client.get(api_url).headers(headers).send().await?;

    if response.status().is_success() {
        println!("Configuration fetched successfully.");
        let config: ConfigFile = response.json().await?;
        // println!("ConfigurationFetch: {:?}", config);
        Ok(config)
    } else {
        Err(anyhow::anyhow!(
            "Failed to fetch configuration: {:?}",
            response.status()
        ))
    }
}
