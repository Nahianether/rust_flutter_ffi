use std::time::Duration;

use anyhow::Result;
use reqwest::Client;

pub async fn check_application_version_from_api() -> Result<String> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    let file_type = if cfg!(target_os = "windows") {
        "exe"
    } else if cfg!(target_os = "macos") {
        "unix"
    } else {
        "unknown"
    };

    let api_url = format!(
        "https://app.trackforce.io/api/TrackerDesktop/CheckLatestVersion?fileType={}",
        file_type
    );

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let response = client.get(api_url).headers(headers).send().await?;

    if response.status().is_success() {
        println!("Check Application Version fetched successfully.");
        println!("Response status check version: {:?}", response.status());
        let response_text = response.text().await?;
        println!("Response status check version: {:?}", response_text);
        let version_data: String = serde_json::from_str(&response_text)?;
        println!("Application Version: {:?}", version_data.to_string());
        Ok(version_data.to_string())
    } else {
        Err(anyhow::anyhow!(
            "Failed to fetch Check Application Version: {:?}",
            response.status()
        ))
    }
}
