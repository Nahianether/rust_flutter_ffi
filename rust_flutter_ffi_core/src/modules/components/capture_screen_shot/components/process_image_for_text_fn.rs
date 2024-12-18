// use anyhow::Result;
// use std::path::Path;

// use crate::models::tracforce_post_model::TextFromImage;
// use leptess::LepTess;
// use regex::Regex;

// pub async fn process_image_for_text(file_path: &Path) -> Result<TextFromImage> {
//     let domain_regex = Regex::new(r"(https?://)?([a-zA-Z0-9-]+\.(com|io|net|edu|dev|info|org|gov|xyz|yt|ai|gg|app|tech|inc|me|pro|live|art|blog|locker|one|is|cc|so|ac|cx|sh|bd))").expect("Failed to compile domain regex");
//     #[cfg(target_os = "windows")]
//     // std::env::set_var("TESSDATA_PREFIX", "C:\\Program Files\\Tesseract-OCR\\tessdata");
//     std::env::set_var("TESSDATA_PREFIX", "./tessdata");

//     #[cfg(target_os = "macos")]
//     let mut lt = LepTess::new(None, "eng")?;
//     #[cfg(target_os = "windows")]
//     // let mut lt = LepTess::new(Some("C:\\Program Files\\Tesseract-OCR\\tessdata"), "eng").expect("Failed to initialize Tesseract");
//     let mut lt = LepTess::new(Some("./tessdata"), "eng").expect("Failed to initialize Tesseract");

//     lt.set_image(file_path.to_str().unwrap())?;

//     // Extract text
//     let text = lt.get_utf8_text()?;
//     if text.trim().is_empty() {
//         return Err(anyhow::anyhow!("No text found in image"));
//     }

//     // Extract domains using regex
//     let domains_found = domain_regex
//         .captures_iter(&text)
//         .filter_map(|cap| cap.get(2).map(|m| m.as_str().to_string()))
//         .collect::<Vec<String>>();

//     let image_name = file_path.file_name().unwrap().to_string_lossy().to_string();
//     let text_from_image = TextFromImage {
//         image_name,
//         detected_domain: domains_found,
//     };

//     println!("OCR text and domains found in image: {:?}", text_from_image);
//     Ok(text_from_image)
// }
