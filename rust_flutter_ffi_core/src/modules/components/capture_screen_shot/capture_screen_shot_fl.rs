use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use image::io::Reader as ImageReader;
use image::jpeg::JpegEncoder;
use reqwest::Client;
use screenshots::Screen;
use serde_json::json;
use std::env;
use std::path::Path;
// use std::sync::{Arc, Mutex};
use tokio::fs::{self, create_dir_all, remove_file};
use tokio::time::{sleep, Duration};

use crate::modules::components::log_error::log_error_fl::log_error_fn;
// use crate::{DESKTOP_DETAILS_DATA, DESKTOP_DETAILS_ID};

pub async fn take_screenshots_fn(interval: u64, token: String) -> Result<()> {
    let mut counter = 1;

    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            log_error_fn(&format!("Failed to get executable path: {:?}", e));
            println!("Failed to get executable path");
            return Err(anyhow::anyhow!("Failed to get executable path"));
        }
    };

    let exe_dir = match exe_path.parent() {
        Some(dir) => dir.to_path_buf(),
        None => {
            log_error_fn("Failed to get executable directory.");
            println!("Failed to get executable directory");
            return Err(anyhow::anyhow!("Failed to get executable directory"));
        }
    };

    let screenshot_dir = exe_dir.join("screenshot");
    if !screenshot_dir.exists() {
        if let Err(e) = create_dir_all(&screenshot_dir).await {
            log_error_fn(&format!("Failed to create screenshot directory: {:?}", e));
            return Err(anyhow::anyhow!("Failed to create screenshot directory"));
        }
    }

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    loop {
        let mut screenshots_json_string = vec![];

        if let Ok(screens) = Screen::all() {
            for (i, scr) in screens.iter().enumerate() {
                let file_name = format!("screenshots{}_monitor{}.png", counter, i + 1);
                let file_path = screenshot_dir.join(&file_name);

                match scr.capture() {
                    Ok(image) => {
                        if image.save(&file_path).is_ok() {
                            if let Ok(base64_image) = image_to_base64_with_resize(&file_path).await
                            {
                                screenshots_json_string.push(json!({
                                    "strImageBase64": base64_image,
                                    "strFileName": file_name,
                                }));
                            }
                            println!("Screenshot captured successfully: {}", file_name);
                        }
                    }
                    Err(e) => log_error_fn(&format!("Failed to capture screenshot: {:?}", e)),
                }
            }
        }

        // Send the screenshots to the API together
        if let Err(e) = send_payload_data_to_api(&client, &screenshots_json_string, &token).await {
            log_error_fn(&format!("Failed to send screenshots to API: {:?}", e));
        } else {
            // Delete all files in the screenshot directory after successful API response
            if let Err(e) = clear_screenshot_folder(&screenshot_dir).await {
                log_error_fn(&format!("Failed to clear screenshot folder: {:?}", e));
            } else {
                println!("Successfully cleared screenshot folder after API success.");
            }
        }

        counter += 1;
        sleep(Duration::from_secs(interval)).await;
    }
}

async fn image_to_base64_with_resize(
    file_path: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    // Open and decode the image file, keeping the original dimensions
    let image = ImageReader::open(file_path)?.decode()?.into_rgb8();

    // Prepare buffer to store the JPEG-encoded image
    let mut buffer = Vec::new();

    // Use the JPEG encoder with specified quality to compress the image
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 3); // Use quality parameter to adjust resolution
    encoder.encode_image(&image)?;

    // Convert the compressed image buffer to a Base64 string
    Ok(general_purpose::STANDARD.encode(&buffer))
}

async fn send_payload_data_to_api(
    client: &Client,
    screenshots: &[serde_json::Value],
    token: &String,
) -> Result<()> {
    let api_url = "https://app.trackforce.io/api/Tracker/TrackingData";
    // let api_url = "https://localhost:7020/api/Tracker/TrackingData";

    // let desktop_details_id = Arc::new(Mutex::new(DESKTOP_DETAILS_ID.lock().unwrap().clone()));
    // let desktop_details_name = Arc::new(Mutex::new(DESKTOP_DETAILS_DATA.lock().unwrap().clone()));

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let payload = json!({
        "intTrackingTypeId": 3,
        "strTrackingType": "Screenshot",
        "strToken": token,
        "strJsonString": serde_json::to_string(&screenshots).unwrap_or_else(|_| "[]".to_string()),
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
            println!("Screenshot sent successfully to the API.");
            Ok(())
        }
        Ok(resp) => Err(anyhow::anyhow!("Failed with status: {:?}", resp.status())),
        Err(e) => {
            println!("Failed to send Screenshot to the API: {:?}", e);
            Err(anyhow::Error::new(e))
        }
    }
}

async fn clear_screenshot_folder(screenshot_dir: &Path) -> Result<(), std::io::Error> {
    let mut entries = fs::read_dir(screenshot_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        if entry.path().is_file() {
            remove_file(entry.path()).await?;
        }
    }
    Ok(())
}
