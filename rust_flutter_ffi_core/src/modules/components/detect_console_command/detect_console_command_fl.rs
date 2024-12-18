use std::env;

use anyhow::Result;
use reqwest::Client;
use screenshots::Screen;
use tokio::fs::{create_dir_all, remove_file};
use tokio::time::Duration;

use crate::api::send_console_data_to_api_fl::send_console_data_to_api_fn;
use crate::components::convert_screenshot_to_base64_fl::image_to_base64_with_resize;
use crate::models::tracforce_post_model::ConsoleCapture;
use crate::modules::components::get_active_window_with_time::get_active_window_with_time_fl::get_active_window_title;
use crate::modules::components::log_error::log_error_fl::log_error_fn;

pub async fn detect_console_command(x: i32, token: &str) -> Result<()> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    loop {
        let active_window_title = match get_active_window_title() {
            Some(title) => title,
            None => {
                println!("No active window title found.");
                tokio::time::sleep(Duration::from_secs(x as u64)).await;
                continue;
            }
        };

        println!("Active window title: {:?}", active_window_title);

        if [
            "cmd",
            "powershell",
            "bash",
            "zsh",
            "terminal",
            "stable",
            "Command Prompt",
        ]
        .iter()
        .any(|&cmd| active_window_title.to_lowercase().contains(cmd))
        {
            println!("Console command detected.");

            let exe_path = env::current_exe()?;
            let exe_dir = exe_path
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))?;
            let screenshot_dir = exe_dir.join("screenshot_for_console");

            if !screenshot_dir.exists() {
                create_dir_all(&screenshot_dir).await?;
            }

            let display_info =
                Screen::all().map_err(|_| anyhow::anyhow!("Failed to get display info"))?;
            let screen = display_info
                .first()
                .ok_or_else(|| anyhow::anyhow!("No display found"))?;
            let file_name = "screenshot_console.png";
            let file_path = screenshot_dir.join(&file_name);

            match screen.capture() {
                Ok(image) => {
                    if let Err(e) = image.save(&file_path) {
                        println!("Failed to save screenshot: {:?}", e);
                        log_error_fn(format!("Failed to save screenshot: {:?}", e).as_str());
                        tokio::time::sleep(Duration::from_secs(x as u64)).await;
                        continue;
                    }
                    println!("Screenshot captured successfully: {}", file_name);

                    match image_to_base64_with_resize(&file_path).await {
                        Ok(base64_image) => {
                            let console_payload = ConsoleCapture {
                                image_path: base64_image,
                                body: active_window_title,
                            };

                            if let Err(e) =
                                send_console_data_to_api_fn(&client, &console_payload, token).await
                            {
                                println!("Failed to send screenshot to API: {:?}", e);
                                log_error_fn(
                                    format!("Failed to send screenshot to API: {:?}", e).as_str(),
                                );
                            }
                        }
                        Err(e) => {
                            println!("Failed to convert screenshot to base64: {:?}", e);
                            log_error_fn(
                                format!("Failed to convert screenshot to base64: {:?}", e).as_str(),
                            );
                        }
                    }

                    if let Err(e) = remove_file(&file_path).await {
                        println!("Failed to delete screenshot file: {:?}", e);
                        log_error_fn(format!("Failed to delete screenshot file: {:?}", e).as_str());
                    } else {
                        println!("Screenshot file deleted: {}", file_name);
                    }
                }
                Err(e) => {
                    println!("Failed to capture screenshot: {:?}", e);
                    log_error_fn(format!("Failed to capture screenshot: {:?}", e).as_str());
                }
            }
        } else {
            println!("No console command detected.");
        }

        tokio::time::sleep(Duration::from_secs(x as u64)).await;
    }
}
