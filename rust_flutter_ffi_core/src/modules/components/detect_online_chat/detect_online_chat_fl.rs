// #[cfg(unix)]
// use anyhow::Result;
// #[cfg(unix)]
// use reqwest::Client;
// #[cfg(unix)]
// use screenshots::Screen;
// #[cfg(unix)]
// use std::env;
// #[cfg(unix)]
// use tokio::fs::create_dir_all;
// #[cfg(unix)]
// use tokio::time::Duration;

// #[cfg(unix)]
// pub async fn detect_online_chat_fn(interval: u64, token: String) -> Result<()> {
//     use std::fs::remove_file;

//     use log::error;

//     use crate::{
//         api::send_detected_chat_data_to_api_fl::send_detect_chat_to_api_fn,
//         components::convert_screenshot_to_base64_fl::image_to_base64_with_resize,
//         models::tracforce_post_model::OnlineMessageDetect,
//         modules::components::detect_online_chat::components::check_image_contains_chat_fl::check_image_contains_chat_fn,
//     };

//     let mut counter = 1;
//     let exe_path = env::current_exe()?;
//     let exe_dir = exe_path
//         .parent()
//         .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))?
//         .to_path_buf();
//     let screenshot_dir = exe_dir.join("screenshot_for_online_chat");

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

//                 if let Ok(image) = screen.capture() {
//                     if image.save(&file_path).is_ok() {
//                         println!("Screenshot captured successfully: {}", file_name);

//                         let online_chat = check_image_contains_chat_fn(&file_path).await;

//                         if online_chat.len() > 0 {
//                             let online_chat = online_chat
//                                 .iter()
//                                 .map(|s| s.to_string())
//                                 .collect::<Vec<String>>();
//                             match image_to_base64_with_resize(&file_path).await {
//                                 Ok(base64_image) => {
//                                     let online_chat = OnlineMessageDetect {
//                                         image_path: base64_image,
//                                         message: online_chat,
//                                     };
//                                     if let Err(e) =
//                                         send_detect_chat_to_api_fn(&client, &online_chat, &token)
//                                             .await
//                                     {
//                                         eprintln!(
//                                             "Failed to send detected Chat image to API: {:?}",
//                                             e
//                                         );
//                                     } else {
//                                         println!("Chat detected and sent to API");
//                                     }
//                                 }
//                                 Err(e) => {
//                                     error!("Failed to convert screenshot to base64: {:?}", e);
//                                 }
//                             }

//                             remove_file(&file_path)?;
//                             println!("Screenshot file deleted: {}", file_name);
//                         } else {
//                             println!("No social media found in screenshot: {}", file_name);
//                         }
//                     }
//                 }
//             }
//         }
//         counter += 1;
//         tokio::time::sleep(Duration::from_secs(interval)).await;
//     }
// }
