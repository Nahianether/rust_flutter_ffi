// #[cfg(unix)]
// use crate::models::tracforce_post_model::MeetingParticipant;
// #[cfg(unix)]
// use anyhow::Result;
// #[cfg(unix)]
// use leptess::LepTess;
// #[cfg(unix)]
// use std::path::Path;

// #[cfg(unix)]
// pub async fn process_image_for_meeting_fn(file_path: &Path) -> Result<Vec<MeetingParticipant>> {
//     let mut participants = Vec::new();

//     #[cfg(target_os = "windows")]
//     std::env::set_var("TESSDATA_PREFIX", "./tessdata");

//     #[cfg(target_os = "macos")]
//     let mut lt = LepTess::new(None, "eng")?;
//     #[cfg(target_os = "windows")]
//     let mut lt = LepTess::new(Some("./tessdata"), "eng").expect("Failed to initialize Tesseract");
//     lt.set_image(file_path.to_str().unwrap())?;

//     let text = lt.get_utf8_text()?;

//     for line in text.lines() {
//         let name = line.trim().to_string();
//         if !name.is_empty() {
//             if is_valid_participant_name(&name) {
//                 participants.push(MeetingParticipant { name });
//             }
//         }
//     }

//     Ok(participants)
// }

// fn is_valid_participant_name(name: &str) -> bool {
//     if name.len() < 3 {
//         return false;
//     }

//     if name.chars().filter(|c| c.is_numeric()).count() > 2 {
//         return false;
//     }

//     if name
//         .chars()
//         .filter(|c| !c.is_alphanumeric() && !c.is_whitespace())
//         .count()
//         > 2
//     {
//         return false;
//     }

//     if !name.contains(' ') {
//         return false;
//     }

//     true
// }
