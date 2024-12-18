// this function is for generate output when event is triggered

// #[cfg(windows)]
// extern crate winapi;

// use std::collections::HashMap;
// use std::ffi::OsString;
// #[cfg(windows)]
// use std::os::windows::ffi::OsStringExt;
// use std::thread::sleep;
// use std::time::{Duration, Instant};
// #[cfg(windows)]
// use winapi::um::winuser::{GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW};

// fn get_active_window_title() -> Option<String> {
//     unsafe {
//         let hwnd = GetForegroundWindow();
//         if hwnd.is_null() {
//             return None; // No active window
//         }

//         let length = GetWindowTextLengthW(hwnd);
//         if length == 0 {
//             return None; // No window title or failed to retrieve it
//         }

//         let mut buffer: Vec<u16> = Vec::with_capacity((length + 1) as usize);
//         buffer.set_len(length as usize);

//         GetWindowTextW(hwnd, buffer.as_mut_ptr(), length + 1);

//         let os_string = OsString::from_wide(&buffer);
//         let window_title = os_string.to_string_lossy().into_owned();

//         Some(window_title)
//     }
// }

// fn update_window_time(
//     last_window: &Option<String>,
//     last_active_time: Instant,
//     window_times: &mut HashMap<String, Duration>,
// ) {
//     if let Some(last_window_title) = last_window {
//         let time_spent = last_active_time.elapsed();
//         let total_time = window_times
//             .entry(last_window_title.clone())
//             .or_insert(Duration::new(0, 0));
//         *total_time += time_spent;
//         println!(
//             "Spent {:?} on window '{}'.",
//             time_spent, last_window_title
//         );
//     }
// }

// fn print_total_time(window_times: &HashMap<String, Duration>) {
//     for (window_title, time_spent) in window_times {
//         println!("Total time on '{}': {:?}", window_title, time_spent);
//     }
// }

// pub fn monitor_active_window_fn() {
//     let mut window_times: HashMap<String, Duration> = HashMap::new();
//     let mut last_window: Option<String> = None;
//     let mut last_active_time = Instant::now();

//     loop {
//         if let Some(current_window) = get_active_window_title() {
//             if Some(current_window.clone()) != last_window {
//                 update_window_time(&last_window, last_active_time, &mut window_times);

//                 last_window = Some(current_window.clone());
//                 last_active_time = Instant::now();
//                 println!("Switched to window '{}'", current_window);
//             }
//         }

//         print_total_time(&window_times);

//         sleep(Duration::from_secs(30));
//     }
// }

// --------------------------------------------------------------------------------------------------------------------

// this function is for generate output all the time and all values

use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::api::get_active_window_with_time_api_fl::send_active_window_data_to_api;
use crate::modules::components::log_error::log_error_fl::log_error_fn;
use crate::ActiveWindowWithTime;

pub fn get_active_window_title() -> Option<String> {
    #[cfg(windows)]
    return get_active_window_title_windows();

    #[cfg(target_os = "macos")]
    return get_active_window_title_macos();

    #[cfg(target_os = "linux")]
    return get_active_window_title_linux();
}

// Windows-specific implementation
#[cfg(windows)]
fn get_active_window_title_windows() -> Option<String> {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};
    use winapi::um::winuser::{GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW};

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            log_error_fn("Failed to get foreground window.");
            return None;
        }

        let length = GetWindowTextLengthW(hwnd);
        if length == 0 {
            log_error_fn("Failed to get window title length.");
            return None;
        }

        let mut buffer: Vec<u16> = vec![0; length as usize + 1];
        GetWindowTextW(hwnd, buffer.as_mut_ptr(), length + 1);

        let os_string = OsString::from_wide(&buffer);
        let window_title = os_string.to_string_lossy().into_owned();

        if window_title.is_empty() {
            log_error_fn("Window title is empty.");
        }

        Some(window_title)
    }
}

// macOS-specific implementation
#[cfg(target_os = "macos")]
fn get_active_window_title_macos() -> Option<String> {
    let output = match std::process::Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to get name of application processes whose frontmost is true")
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            log_error_fn(&format!("Failed to run osascript command: {:?}", e));
            return None;
        }
    };

    let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if title.is_empty() {
        log_error_fn("No active window title found on macOS.");
        None
    } else {
        Some(title)
    }
}

// Linux-specific implementation
#[cfg(target_os = "linux")]
fn get_active_window_title_linux() -> Option<String> {
    let output = match std::process::Command::new("xdotool")
        .arg("getactivewindow")
        .arg("getwindowname")
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            log_error_fn(&format!("Failed to run xdotool command: {:?}", e));
            return None;
        }
    };

    let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if title.is_empty() {
        log_error_fn("No active window title found on Linux.");
        None
    } else {
        Some(title)
    }
}

pub async fn monitor_active_window_fn(interval_secs: u64, token: String) -> Result<()> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    let mut window_times: HashMap<String, Duration> = HashMap::new();
    let mut last_window: Option<String> = None;
    let mut last_active_time = Instant::now();
    let interval = Duration::from_secs(interval_secs);

    loop {
        if let Some(current_window) = get_active_window_title() {
            if Some(&current_window) != last_window.as_ref() {
                if let Some(last_window_title) = last_window.take() {
                    let time_spent = last_active_time.elapsed();
                    *window_times
                        .entry(last_window_title.clone())
                        .or_insert(Duration::new(0, 0)) += time_spent;

                    let window_data = ActiveWindowWithTime {
                        window_name: last_window_title.clone(),
                        time_spent: time_spent.as_secs(),
                    };

                    if !window_data.window_name.is_empty() {
                        if let Err(e) =
                            send_active_window_data_to_api(&client, window_data, &token).await
                        {
                            log_error_fn(&format!(
                                "Failed to send active window data to API: {:?}",
                                e
                            ));
                        }
                        continue;
                    }

                    println!("Spent {:?} on window '{}'", time_spent, last_window_title);
                }

                last_window = Some(current_window.clone());
                last_active_time = Instant::now();
                println!("Switched to new window '{}'", current_window);
            }
        } else {
            log_error_fn("Failed to get active window title.");
        }

        sleep(interval).await;
    }
}

// Main function to monitor the active window and the time spent on it
// pub fn monitor_active_window_fn(shared_payload: Arc<Mutex<ApiPayload>>, x: i32) {
//     let mut window_times: HashMap<String, Duration> = HashMap::new();
//     let mut last_window: Option<String> = None;
//     let mut last_active_time = Instant::now();
//     let mut update_buffer = Vec::new();

//     loop {
//         match get_active_window_title() {
//             Some(current_window) => {
//                 if Some(current_window.clone()) != last_window {
//                     if let Some(last_window_title) = last_window.clone() {
//                         let time_spent = last_active_time.elapsed();
//                         let total_time = window_times.entry(last_window_title.clone()).or_insert(Duration::new(0, 0));
//                         *total_time += time_spent;

//                         // Add data to the local buffer instead of directly to shared_payload
//                         update_buffer.push(ActiveWindowWithTime {
//                             window_name: last_window_title.clone(),
//                             time_spent: time_spent.as_secs(),
//                         });

//                         println!(
//                             "Spent {:?} on window '{}'.",
//                             time_spent, last_window.unwrap()
//                         );
//                     }

//                     last_window = Some(current_window.clone());
//                     last_active_time = Instant::now();
//                     println!("Switched to window '{}'", current_window);
//                 }
//             }
//             None => {
//                 log_error_fn("No active window or failed to get window title.");
//                 println!("No active window or failed to get window title.");
//             }
//         }

//         // Update shared_payload once per loop with the buffered data
//         if !update_buffer.is_empty() {
//             let mut payload = shared_payload.lock().unwrap();
//             payload.active_window_with_time.extend(update_buffer.drain(..));
//         }

//         sleep(Duration::from_secs(x as u64));

//         for (window_title, time_spent) in &window_times {
//             println!("Total time on '{}': {:?}", window_title, time_spent);
//         }
//     }
// }
