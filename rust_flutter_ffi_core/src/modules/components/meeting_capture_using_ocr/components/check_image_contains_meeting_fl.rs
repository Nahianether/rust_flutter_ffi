// #[cfg(unix)]
// use leptess::LepTess;
// #[cfg(unix)]
// use regex::Regex;
// #[cfg(unix)]
// use std::path::Path;

// #[cfg(unix)]
// pub async fn check_image_contains_meeting_fn(file_path: &Path) -> bool {
//     let url_regex = Regex::new(r"(https?://)?([a-zA-Z0-9.-]*meet\.google\.com)")
//         .expect("Failed to compile domain regex");
//     #[cfg(target_os = "windows")]
//     // std::env::set_var("TESSDATA_PREFIX", "C:\\Program Files\\Tesseract-OCR\\tessdata");
//     std::env::set_var("TESSDATA_PREFIX", "./tessdata");

//     #[cfg(target_os = "macos")]
//     let mut lt = LepTess::new(None, "eng").expect("Failed to initialize Tesseract");
//     #[cfg(target_os = "windows")]
//     // let mut lt = LepTess::new(Some("C:\\Program Files\\Tesseract-OCR\\tessdata"), "eng").expect("Failed to initialize Tesseract");
//     let mut lt = LepTess::new(Some("./tessdata"), "eng").expect("Failed to initialize Tesseract");

//     lt.set_image(file_path.to_str().unwrap())
//         .expect("Failed to set image");

//     let text = lt
//         .get_utf8_text()
//         .expect("Failed to extract text from image");
//     if text.trim().is_empty() {
//         return false;
//     }

//     let domains_found = url_regex
//         .captures_iter(&text)
//         .filter_map(|cap| cap.get(2).map(|m| m.as_str().to_string()))
//         .collect::<Vec<String>>();

//     if domains_found.is_empty() {
//         println!("No meeting found in image");
//         return false;
//     }

//     println!("Meeting found in image: {:?}", domains_found);
//     return true;
// }
