// #[cfg(unix)]
// use anyhow::Result;
// #[cfg(unix)]
// use leptess::LepTess;
// #[cfg(unix)]
// use std::path::Path;

// #[cfg(unix)]
// pub async fn process_image_for_email_fn(file_path: &Path) -> Result<String> {
//     #[cfg(target_os = "windows")]
//     std::env::set_var("TESSDATA_PREFIX", "./tessdata");

//     #[cfg(target_os = "macos")]
//     let mut lt = LepTess::new(None, "eng")?;
//     #[cfg(target_os = "windows")]
//     let mut lt = LepTess::new(Some("./tessdata"), "eng").expect("Failed to initialize Tesseract");

//     lt.set_image(file_path.to_str().unwrap())?;

//     let text = lt.get_utf8_text()?;
//     if text.trim().is_empty() {
//         return Err(anyhow::anyhow!("Failed to extract text from image"));
//     }

//     let mut body_lines = vec![];

//     for line in text.lines() {
//         body_lines.push(line.trim().to_string());
//     }

//     let body = if body_lines.is_empty() {
//         None
//     } else {
//         Some(body_lines.join("\n"))
//     };

//     Ok(body.unwrap_or_default())
// }
