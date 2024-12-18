// #[cfg(unix)]
// use anyhow::Result;
// #[cfg(unix)]
// use image::GenericImageView;
// #[cfg(unix)]
// use screenshots::Screen;
// #[cfg(unix)]
// use std::env;
// #[cfg(unix)]
// use std::path::Path;
// #[cfg(unix)]
// use tokio::fs::create_dir_all;
// #[cfg(unix)]
// use tokio::time::Duration;

// #[cfg(unix)]
// use crate::modules::components::meeting_capture_using_ocr::components::check_image_contains_meeting_fl::check_image_contains_meeting_fn;
// #[cfg(unix)]
// use crate::modules::components::meeting_capture_using_ocr::components::process_image_for_meeting_fl::process_image_for_meeting_fn;
// // use crate::api::send_screenshot_and_text_api_fl::send_screenshot_to_api;
// // use crate::models::tracforce_post_model::TextFromImage;
// // use crate::api::send_screenshot_and_text_api_fl::send_text_to_api;
// // use crate::components::convert_screenshot_to_base64_fl::image_to_base64_with_resize;
// #[cfg(unix)]
// pub async fn capture_meeting_using_ocr_fn(interval: u64, token: String) -> Result<()> {
//     use std::fs::remove_file;

//     use log::error;
//     use reqwest::Client;

//     use crate::{
//         api::send_detect_meeting_data_to_api_fl::send_detect_meeting_data_to_api_fn,
//         components::convert_screenshot_to_base64_fl::image_to_base64_with_resize,
//         models::tracforce_post_model::MeetingDetails,
//     };

//     let mut counter = 1;
//     let exe_path = env::current_exe()?;
//     let exe_dir = exe_path
//         .parent()
//         .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))?
//         .to_path_buf();
//     let screenshot_dir = exe_dir.join("screenshot_for_meeting");

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

//                         let is_meeting = check_image_contains_meeting_fn(&file_path).await;

//                         if is_meeting {
//                             if let Err(e) = crop_image(&file_path) {
//                                 eprintln!("Failed to crop image: {:?}", e);
//                             }

//                             match process_image_for_meeting_fn(&file_path).await {
//                                 Ok(meeting_data_from_image) => {
//                                     match image_to_base64_with_resize(&file_path).await {
//                                         Ok(base64_image) => {
//                                             let meeting_body = MeetingDetails {
//                                                 participant: meeting_data_from_image,
//                                                 image_path: base64_image,
//                                             };
//                                             if let Err(e) = send_detect_meeting_data_to_api_fn(
//                                                 &client,
//                                                 &meeting_body,
//                                                 &token,
//                                             )
//                                             .await
//                                             {
//                                                 eprintln!(
//                                                     "Failed to send detected meeting to API: {:?}",
//                                                     e
//                                                 );
//                                             } else {
//                                                 println!("OCR meeting found in image");
//                                             }
//                                         }
//                                         Err(e) => {
//                                             error!(
//                                                 "Failed to convert screenshot to base64: {:?}",
//                                                 e
//                                             );
//                                         }
//                                     }
//                                 }
//                                 Err(e) => {
//                                     eprintln!("Failed to process meeting image for text: {:?}", e);
//                                 }
//                             }

//                             remove_file(&file_path)?;
//                             println!("Screenshot file deleted: {}", file_name);
//                         }
//                     }
//                 }
//             }
//         }
//         counter += 1;
//         tokio::time::sleep(Duration::from_secs(interval)).await;
//     }
// }

// #[cfg(unix)]
// fn crop_image(input_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//     let mut img = image::open(input_path)?;

//     let (width, height) = img.dimensions();

//     let crop_x = (width as f32 * 0.0) as u32;
//     let crop_y = (height as f32 * 0.20) as u32;
//     let crop_width = width - crop_x;
//     let crop_height = height - crop_y;

//     let cropped = img.crop(crop_x, crop_y, crop_width, crop_height);

//     cropped.save(input_path)?;

//     println!("Image cropped successfully. Saved to: {:?}", input_path);

//     Ok(())
// }
