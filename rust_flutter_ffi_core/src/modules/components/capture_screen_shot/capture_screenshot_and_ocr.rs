// use anyhow::Result;
// use reqwest::Client;
// use screenshots::Screen;
// use serde_json::json;
// use std::env;
// use tokio::fs::{create_dir_all, remove_file};
// use tokio::time::Duration;

// use crate::api::send_screenshot_and_text_api_fl::send_screenshot_to_api;
// // use crate::models::tracforce_post_model::TextFromImage;
// // use crate::api::send_screenshot_and_text_api_fl::send_text_to_api;
// use crate::components::convert_screenshot_to_base64_fl::image_to_base64_with_resize;

// pub async fn capture_and_process_screenshot_fn(interval: u64, token: String) -> Result<()> {
//     let mut counter = 1;
//     let exe_path = env::current_exe()?;
//     let exe_dir = exe_path
//         .parent()
//         .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))?
//         .to_path_buf();
//     let screenshot_dir = exe_dir.join("screenshot");

//     if !screenshot_dir.exists() {
//         create_dir_all(&screenshot_dir).await?;
//     }

//     let client = Client::builder()
//         .danger_accept_invalid_certs(true)
//         .connect_timeout(Duration::from_secs(15))
//         .timeout(Duration::from_secs(60))
//         .build()
//         .expect("Failed to create reqwest client");

//     loop {
//         if let Ok(screens) = Screen::all() {
//             for (i, screen) in screens.iter().enumerate() {
//                 let file_name = format!("screenshot{}_monitor{}.png", counter, i + 1);
//                 let file_path = screenshot_dir.join(&file_name);

//                 // Capture screenshot and save it
//                 if let Ok(image) = screen.capture() {
//                     if image.save(&file_path).is_ok() {
//                         println!("Screenshot captured successfully: {}", file_name);

//                         // Perform OCR to detect text from the image
//                         // match process_image_for_text(&file_path, &domain_regex).await {
//                         //     Ok(text_from_image) => {
//                         //         // Send OCR data to API
//                         //         send_text_to_api(&client, &text_from_image, &token).await?;
//                         //     }
//                         //     Err(e) => {
//                         //         eprintln!("Failed to process image for text: {:?}", e);
//                         //     }
//                         // }

//                         // Send screenshot data to API
//                         if let Ok(base64_image) = image_to_base64_with_resize(&file_path).await {
//                             let screenshot_data = json!([{
//                                 "strImageBase64": base64_image,
//                                 "strFileName": file_name,
//                             }]);
//                             send_screenshot_to_api(&client, &screenshot_data, &token).await?;
//                         }

//                         // Remove screenshot file after processing
//                         remove_file(&file_path).await?;
//                         println!("Screenshot file deleted: {}", file_name);
//                     }
//                 }
//             }
//         }
//         counter += 1;
//         tokio::time::sleep(Duration::from_secs(interval)).await;
//     }
// }

// --------------------------------------------------------------------------------------------------------

// use anyhow::Result;
// use reqwest::Client;
// use screenshots::Screen;
// use serde_json::json;
// use std::env;
// use tokio::fs::{create_dir_all, remove_file};
// use tokio::time::Duration;
// use log::{error, info};
// use simple_logger::SimpleLogger;

// use crate::api::send_screenshot_and_text_api_fl::send_screenshot_to_api;
// use crate::components::convert_screenshot_to_base64_fl::image_to_base64_with_resize;

// pub async fn capture_and_process_screenshot_fn(interval: u64, token: String) -> Result<()> {
//     SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

//     let mut counter = 1;
//     let exe_path = env::current_exe()?;
//     let exe_dir = exe_path
//         .parent()
//         .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))?
//         .to_path_buf();
//     let screenshot_dir = exe_dir.join("screenshot");

//     if !screenshot_dir.exists() {
//         create_dir_all(&screenshot_dir).await?;
//     }

//     let client = Client::builder()
//         .danger_accept_invalid_certs(true)
//         .connect_timeout(Duration::from_secs(15))
//         .timeout(Duration::from_secs(60))
//         .build()
//         .expect("Failed to create reqwest client");

//     loop {
//         if let Ok(screens) = Screen::all() {
//             for (i, screen) in screens.iter().enumerate() {
//                 let file_name = format!("screenshot{}_monitor{}.png", counter, i + 1);
//                 let file_path = screenshot_dir.join(&file_name);

//                 // Capture screenshot and save it
//                 match screen.capture() {
//                     Ok(image) => {
//                         if let Err(e) = image.save(&file_path) {
//                             error!("Failed to save screenshot: {:?}", e);
//                             continue;
//                         }
//                         info!("Screenshot captured successfully: {}", file_name);

//                         // Perform OCR to detect text from the image
//                         // match process_image_for_text(&file_path, &domain_regex).await {
//                         //     Ok(text_from_image) => {
//                         //         // Send OCR data to API
//                         //         send_text_to_api(&client, &text_from_image, &token).await?;
//                         //     }
//                         //     Err(e) => {
//                         //         eprintln!("Failed to process image for text: {:?}", e);
//                         //     }
//                         // }

//                         // Send screenshot data to API
//                         match image_to_base64_with_resize(&file_path).await {
//                             Ok(base64_image) => {
//                                 let screenshot_data = json!([{
//                                     "strImageBase64": base64_image,
//                                     "strFileName": file_name,
//                                 }]);
//                                 if let Err(e) = send_screenshot_to_api(&client, &screenshot_data, &token).await {
//                                     error!("Failed to send screenshot to API: {:?}", e);
//                                 }
//                             }
//                             Err(e) => {
//                                 error!("Failed to convert screenshot to base64: {:?}", e);
//                             }
//                         }

//                         // Remove screenshot file after processing
//                         if let Err(e) = remove_file(&file_path).await {
//                             error!("Failed to delete screenshot file: {:?}", e);
//                         } else {
//                             info!("Screenshot file deleted: {}", file_name);
//                         }
//                     }
//                     Err(e) => {
//                         error!("Failed to capture screenshot: {:?}", e);
//                     }
//                 }
//             }
//         } else {
//             error!("Failed to get screens for capturing.");
//         }
//         counter += 1;
//         tokio::time::sleep(Duration::from_secs(interval)).await;
//     }
// }

// --------------------------------------------------------------------------------------------------------

use anyhow::Result;
use reqwest::Client;
use screenshots::Screen;
use std::env;
// use std::time::Instant;
use tokio::fs::{create_dir_all, remove_file};
use tokio::time::Duration;

use crate::api::send_screenshot_and_text_api_fl::send_screenshot_to_api;
use crate::components::convert_screenshot_to_base64_fl::image_to_base64_with_resize;
use crate::modules::components::log_error::log_error_fl::log_error_fn;

// #[cfg(target_os = "windows")]
// fn is_system_locked() -> bool {
//     #[cfg(target_os = "windows")]
//     {
//         use winapi::um::winuser::{CloseDesktop, OpenInputDesktop, DESKTOP_SWITCHDESKTOP};
//         unsafe {
//             let desktop_handle = OpenInputDesktop(0, false as i32, DESKTOP_SWITCHDESKTOP);
//             if desktop_handle.is_null() {
//                 return true;
//             }
//             CloseDesktop(desktop_handle);
//             false
//         }
//     }
// }

pub async fn capture_and_process_screenshot_fn(interval: u64, token: String) -> Result<()> {
    let mut counter = 1;
    // #[cfg(target_os = "windows")]
    // let mut lock_start_time: Option<Instant> = None;
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))?
        .to_path_buf();
    let screenshot_dir = exe_dir.join("screenshot");

    if !screenshot_dir.exists() {
        create_dir_all(&screenshot_dir).await?;
    }

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    loop {
        // #[cfg(target_os = "windows")]
        // {
        //     if is_system_locked() {
        //         if lock_start_time.is_none() {
        //             lock_start_time = Some(Instant::now());
        //         }

        //         println!("System is locked. Skipping screenshot capture.");
        //         tokio::time::sleep(Duration::from_secs(interval)).await;
        //         continue;
        //     } else if let Some(start_time) = lock_start_time {
        //         let lock_duration = start_time.elapsed();
        //         println!(
        //             "System unlocked. Total lock time: {} seconds",
        //             lock_duration.as_secs()
        //         );
        //         lock_start_time = None;
        //     }
        // }

        if let Ok(screens) = Screen::all() {
            for (i, screen) in screens.iter().enumerate() {
                let file_name = format!("screenshot{}_monitor{}.png", counter, i + 1);
                let file_path = screenshot_dir.join(&file_name);

                match screen.capture() {
                    Ok(image) => {
                        if let Err(e) = image.save(&file_path) {
                            println!("Failed to save screenshot: {:?}", e);
                            log_error_fn(format!("Failed to save screenshot: {:?}", e).as_str());
                            continue;
                        }
                        println!("Screenshot captured successfully: {}", file_name);

                        // Perform OCR to detect text from the image
                        // match process_image_for_text(&file_path, &domain_regex).await {
                        //     Ok(text_from_image) => {
                        //         // Send OCR data to API
                        //         send_text_to_api(&client, &text_from_image, &token).await?;
                        //     }
                        //     Err(e) => {
                        //         eprintln!("Failed to process image for text: {:?}", e);
                        //     }
                        // }

                        match image_to_base64_with_resize(&file_path).await {
                            Ok(base64_image) => {
                                if let Err(e) =
                                    send_screenshot_to_api(&client, &base64_image, &token).await
                                {
                                    println!("Failed to send screenshot to API: {:?}", e);
                                }
                            }
                            Err(e) => {
                                println!("Failed to convert screenshot to base64: {:?}", e);
                                log_error_fn(
                                    format!("Failed to convert screenshot to base64: {:?}", e)
                                        .as_str(),
                                );
                            }
                        }

                        if let Err(e) = remove_file(&file_path).await {
                            println!("Failed to delete screenshot file: {:?}", e);
                            log_error_fn(
                                format!("Failed to delete screenshot file: {:?}", e).as_str(),
                            );
                        } else {
                            println!("Screenshot file deleted: {}", file_name);
                        }
                    }
                    Err(e) => {
                        println!("Failed to capture screenshot: {:?}", e);
                        log_error_fn(format!("Failed to capture screenshot: {:?}", e).as_str());
                    }
                }
            }
        } else {
            println!("Failed to get screens for capturing.");
            log_error_fn("Failed to get screens for capturing.");
        }
        counter += 1;
        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}
