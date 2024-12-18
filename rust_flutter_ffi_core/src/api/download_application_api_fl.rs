use std::{
    fs::{self, File},
    io::Write,
    // path::Path,
    // sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Result;
use reqwest::Client;

// use crate::SHARED_PAYLOAD;

pub async fn download_application_from_api_fn(employee_id: &String) -> Result<()> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    println!("{:?}", client);

    let api_url = format!(
        "https://app.trackforce.io/api/Document/DownloadExe/{}/windows.exe",
        employee_id
    );

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let response = client.get(api_url).headers(headers).send().await?;

    if response.status().is_success() {
        println!("Application Download API successful.");

        // Get the current executable directory
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))?
            .to_path_buf();

        // Create the new_app directory inside the executable directory
        let new_app_dir = exe_dir.join("new_app");
        if !new_app_dir.exists() {
            fs::create_dir_all(&new_app_dir)?;
        }

        // Create the file to save the downloaded content
        let file_path = new_app_dir.join("1859c1ae-88dc-4ce4-a825-c47d6faa2047_track_force_rs.exe");
        let mut file = File::create(&file_path)?;

        // Write the response content into the file
        let content = response.bytes().await?;
        file.write_all(&content)?;

        println!("File downloaded successfully to: {}", file_path.display());
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Failed to Application Download API: {:?}",
            response.status()
        ))
    }
}
