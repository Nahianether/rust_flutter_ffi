// #[cfg(unix)]
// use leptess::LepTess;
// #[cfg(unix)]
// use regex::Regex;
// use reqwest::Client;
// use serde_json::json;
// use std::env;
// use std::error::Error;
// #[cfg(unix)]
// use tokio::fs;
// use tokio::time::Duration;

// use crate::models::tracforce_post_model::TextFromImage;
// use crate::modules::components::log_error::log_error_fl::log_error_fn;

// #[cfg(unix)]
// pub async fn detect_text_in_image_fn(token: String) -> Result<(), Box<dyn Error>> {
//     let client = Client::builder()
//         .danger_accept_invalid_certs(true)
//         .connect_timeout(Duration::from_secs(15))
//         .timeout(Duration::from_secs(60))
//         .build()
//         .expect("Failed to create reqwest client");

//     let domain_regex = Regex::new(r"(https?://)?([a-zA-Z0-9-]+\.(com|io|net|edu|dev|info|org|gov|xyz|yt|ai|gg|app|tech|inc|me|pro|live|art|blog|locker|one|is|cc|so|ac|cx|sh|bd))")
//         .expect("Failed to compile domain regex");

//     loop {
//         let exe_path = match env::current_exe() {
//             Ok(path) => path,
//             Err(e) => {
//                 log_error_fn(&format!("Failed to get executable path: {}", e));
//                 return Err(Box::new(e));
//             }
//         };

//         let screenshot_dir = exe_path.parent().unwrap().join("screenshot");
//         if !screenshot_dir.exists() {
//             log_error_fn("Screenshot directory not found.");
//             return Ok(());
//         }

//         // Step 1: Read and filter image paths asynchronously
//         let images = match fs::read_dir(&screenshot_dir).await {
//             Ok(mut entries) => {
//                 let mut images = Vec::new();
//                 while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
//                     if entry.path().extension().unwrap_or_default() == "png" {
//                         images.push(entry.path());
//                     }
//                 }
//                 images
//             }
//             Err(e) => {
//                 log_error_fn(&format!("Failed to read screenshot directory: {}", e));
//                 return Ok(());
//             }
//         };

//         if images.is_empty() {
//             println!("No images found in the screenshot directory.");
//         } else {
//             // Initialize Tesseract once per loop
//             let mut lt = match LepTess::new(None, "eng") {
//                 Ok(engine) => engine,
//                 Err(e) => {
//                     log_error_fn(&format!("Failed to initialize Tesseract: {}", e));
//                     return Ok(());
//                 }
//             };

//             for image_path in images {
//                 if let Err(_) = lt.set_image(image_path.to_str().unwrap()) {
//                     log_error_fn(&format!("Failed to set image: {:?}", image_path));
//                     continue;
//                 }

//                 // Extract text
//                 let text = match lt.get_utf8_text() {
//                     Ok(txt) => txt,
//                     Err(e) => {
//                         log_error_fn(&format!(
//                             "Error extracting text from image {:?}: {}",
//                             image_path, e
//                         ));
//                         continue;
//                     }
//                 };

//                 if text.trim().is_empty() {
//                     continue;
//                 }

//                 // Extract domains using regex
//                 let domains_found = domain_regex
//                     .captures_iter(&text)
//                     .filter_map(|cap| cap.get(2).map(|m| m.as_str().to_string()))
//                     .collect::<Vec<String>>();

//                 let image_name = image_path
//                     .file_name()
//                     .unwrap()
//                     .to_string_lossy()
//                     .to_string();

//                 let text_from_image = TextFromImage {
//                     image_name,
//                     detected_domain: domains_found.clone(),
//                 };

//                 if !domains_found.is_empty() {
//                     println!(
//                         "Domains found in image {:?}: {:?}",
//                         image_path, domains_found
//                     );
//                 }

//                 // Send text and domains to API asynchronously
//                 if let Err(e) = send_text_to_api(&client, &text_from_image, &token).await {
//                     log_error_fn(&format!("Failed to send text data to API: {:?}", e));
//                 }
//             }
//         }

//         // Sleep between iterations asynchronously
//         // sleep(Duration::from_secs(x as u64)).await;
//     }
// }

// async fn send_text_to_api(
//     client: &Client,
//     text_data: &TextFromImage,
//     token: &String,
// ) -> Result<(), Box<dyn Error>> {
//     let api_url = "https://app.trackforce.io/api/Tracker/TrackingData";
//     // let api_url = "https://localhost:7020/api/Tracker/TrackingData";

//     // let desktop_details_id = Arc::new(Mutex::new(DESKTOP_DETAILS_ID.lock().unwrap().clone()));
//     // let desktop_details_name = Arc::new(Mutex::new(DESKTOP_DETAILS_DATA.lock().unwrap().clone()));

//     let mut headers = reqwest::header::HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse()?);

//     let payload = json!({
//         "intTrackingTypeId": 4,
//         "strTrackingType": "TextFromImage",
//         // "strToken": "66ae2993-2224-4868-9cd0-b43c5c59cded",
//         "strToken": token,
//         "strJsonString": serde_json::to_string(&text_data).unwrap_or_else(|_| "[]".to_string()),
//         // "desktopId": desktop_details_id.lock().unwrap().clone(),
//         // "desktopName": desktop_details_name.lock().unwrap().clone(),
//     });

//     let response = client
//         .post(api_url)
//         .headers(headers)
//         .json(&payload)
//         .send()
//         .await;

//     match response {
//         Ok(resp) if resp.status().is_success() => {
//             println!("Image to Text data sent successfully to the API.");
//             Ok(())
//         }
//         Ok(resp) => Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             format!("Failed with status: {:?}", resp.status()),
//         ))),
//         Err(e) => {
//             println!("Failed to send Image to Text to the API: {:?}", e);
//             Err(Box::new(e))
//         }
//     }
// }
