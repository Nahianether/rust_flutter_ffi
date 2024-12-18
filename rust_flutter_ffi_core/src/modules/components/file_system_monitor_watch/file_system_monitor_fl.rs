// #[cfg(windows)]
// extern crate winapi;
// #[cfg(windows)]
// extern crate winreg;

// use anyhow::Result;
// use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
// use std::path::Path;
// use std::sync::mpsc::channel;
// use std::thread;

// pub async fn file_system_monitor_mod_fn(x: u64, token: String) -> Result<()> {
//     println!("Token: {}", token);
//     println!("Duration: {}", x);
//     thread::spawn(|| {
//         let (tx, rx) = channel();

//         let mut watcher = recommended_watcher(tx).expect("Failed to create watcher");

//         #[cfg(windows)]
//         let path = Path::new("D:/");

//         #[cfg(target_os = "macos")]
//         let path = Path::new("/Users/intishar/Downloads/");

//         watcher
//             .watch(path, RecursiveMode::Recursive)
//             .expect("Failed to watch path");

//         loop {
//             match rx.recv() {
//                 Ok(event) => match event {
//                     Ok(Event { kind, paths, .. }) => {
//                         for path in paths {
//                             match kind {
//                                 notify::event::EventKind::Create(_) => {
//                                     println!("File created: {:?}", path)
//                                 }
//                                 notify::event::EventKind::Modify(_) => {
//                                     println!("File modified: {:?}", path)
//                                 }
//                                 notify::event::EventKind::Remove(_) => {
//                                     println!("File deleted: {:?}", path)
//                                 }
//                                 _ => continue,
//                             }
//                         }
//                     }
//                     Err(e) => println!("Watch error: {:?}", e),
//                 },
//                 Err(e) => println!("Channel error: {:?}", e),
//             }
//         }
//     });

//     Ok(())
// }

// ------------------------------------------------------------------------------------------------------------------------------------------------

// use anyhow::Result;
// use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
// use reqwest::Client;
// use std::path::Path;
// use std::sync::Arc;
// use tokio::sync::Mutex;
// // use std::thread;
// use std::time::Duration;
// use tokio::task;
// use tokio::time::sleep;

// use crate::api::send_file_events_to_api_fl::send_file_events_to_api;
// use crate::models::tracforce_post_model::FileMonitoring;

// pub async fn file_system_monitor_mod_fn(x: u64, token: String) -> Result<()> {
//     println!("Token: {}", token);
//     println!("Duration: {}", x);

//     let client = Client::builder()
//         .danger_accept_invalid_certs(true)
//         .connect_timeout(Duration::from_secs(15))
//         .timeout(Duration::from_secs(60))
//         .build()
//         .expect("Failed to create reqwest client");
//     // let events_list = Arc::new(Mutex::new(Vec::<FileMonitoring>::new()));
//     let events_list = Arc::new(Mutex::new(Vec::new()));
//     let events_list_clone = Arc::clone(&events_list);

//     tokio::spawn(async move {
//         let (tx, rx) = std::sync::mpsc::channel();

//         let mut watcher = recommended_watcher(tx).expect("Failed to create watcher");

//         #[cfg(windows)]
//         let path = Path::new("D:/");

//         #[cfg(target_os = "macos")]
//         let path = Path::new("/Users/intishar/Downloads/");

//         watcher
//             .watch(path, RecursiveMode::Recursive)
//             .expect("Failed to watch path");

//         loop {
//             match rx.recv() {
//                 Ok(event) => match event {
//                     Ok(Event { kind, paths, .. }) => {
//                         for path in paths {
//                             let event_type = match kind {
//                                 notify::event::EventKind::Create(_) => "Create",
//                                 notify::event::EventKind::Modify(_) => "Modify",
//                                 notify::event::EventKind::Remove(_) => "Delete",
//                                 _ => continue,
//                             };
//                             let file_event = FileMonitoring {
//                                 event_type: event_type.to_string(),
//                                 file_path: path.to_string_lossy().to_string(),
//                                 timestamp: chrono::Utc::now()
//                                     .with_timezone(
//                                         &chrono::FixedOffset::east_opt(6 * 3600).unwrap(),
//                                     )
//                                     .to_rfc3339(),
//                             };

//                             let mut events = events_list.lock().await;
//                             events.push(file_event.clone());

//                             println!("File event: {:?}", file_event);
//                         }
//                     }
//                     Err(e) => println!("Watch error: {:?}", e),
//                 },
//                 Err(e) => println!("Channel error: {:?}", e),
//             }
//         }
//     });

//     task::spawn(async move {
//         loop {
//             sleep(Duration::from_secs(x)).await;

//             let mut events = events_list_clone.lock().await;
//             if !events.is_empty() {
//                 if let Err(e) = send_file_events_to_api(&client, &events, &token).await {
//                     eprintln!("Failed to send events to API: {:?}", e);
//                 } else {
//                     println!("Successfully sent events to API: {:?}", events);
//                     events.clear();
//                 }
//             }
//         }
//     });

//     Ok(())
// }

// ------------------------------------------------------------------------------------------------------------------------------------------------

use anyhow::Result;
use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
use reqwest::Client;
use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver};
use tokio::time::{sleep, Duration};

use crate::api::send_file_events_to_api_fl::send_file_events_to_api;
use crate::models::tracforce_post_model::{FileMonitoring, FileMonitoringPayload};
use crate::modules::components::log_error::log_error_fl::log_error_fn;

pub async fn file_system_monitor_mod_fn(x: u64, token: String) -> Result<()> {
    let paths = if cfg!(target_os = "windows") {
        vec![PathBuf::from("D:/"), PathBuf::from("E:/")]
    } else if cfg!(target_os = "macos") {
        let home_dir = env::var("HOME").expect("Failed to get home directory");
        vec![
            Path::new(&home_dir).join("Documents"),
            Path::new(&home_dir).join("Downloads"),
        ]
    } else {
        println!("Unsupported platform");
        return Ok(());
    };

    println!("Monitoring paths: {:?}", paths);

    let client = Arc::new(
        Client::builder()
            .danger_accept_invalid_certs(true)
            .connect_timeout(Duration::from_secs(15))
            .timeout(Duration::from_secs(60))
            .build()
            .expect("Failed to create reqwest client"),
    );

    let allowed_extensions = vec![
        "txt", "jpeg", "jpg", "png", "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "webp",
        "gif", "mp4", "mov", "avi", "mkv", "mp3", "wav", "flac", "zip", "rar", "7z", "tar", "gz",
    ];

    let (event_tx, event_rx) = channel(100);

    for path in paths {
        if !path.exists() {
            println!("Path not found, skipping: {}", path.display());
            continue;
        }

        let allowed_extensions_clone = allowed_extensions.clone();
        let event_tx_clone = event_tx.clone();

        tokio::spawn(async move {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher = match recommended_watcher(tx) {
                Ok(w) => w,
                Err(e) => {
                    println!("Failed to create watcher for {}: {:?}", path.display(), e);
                    return;
                }
            };

            if let Err(e) = watcher.watch(&path, RecursiveMode::Recursive) {
                println!(
                    "Failed to watch path {}: {:?}. Skipping...",
                    path.display(),
                    e
                );
                return;
            }

            println!("Started watching path: {}", path.display());

            for event in rx {
                match event {
                    Ok(Event { kind, paths, .. }) => {
                        for path in paths {
                            if let Some(extension) = path.extension() {
                                if !allowed_extensions_clone
                                    .contains(&extension.to_string_lossy().to_lowercase().as_str())
                                {
                                    continue;
                                }
                            } else {
                                continue;
                            }

                            let event_type = match kind {
                                notify::event::EventKind::Create(_) => "Create",
                                notify::event::EventKind::Modify(_) => "Modify",
                                notify::event::EventKind::Remove(_) => "Delete",
                                _ => continue,
                            };

                            let file_event = FileMonitoring {
                                event_type: event_type.to_string(),
                                file_path: path.to_string_lossy().to_string(),
                                timestamp: chrono::Utc::now()
                                    .with_timezone(
                                        &chrono::FixedOffset::east_opt(6 * 3600).unwrap(),
                                    )
                                    .to_rfc3339(),
                            };

                            if let Err(e) = event_tx_clone.send(file_event).await {
                                println!("Failed to send event to channel: {:?}", e);
                            }
                        }
                    }
                    Err(e) => println!("Watch error: {:?}", e),
                }
            }
        });
    }

    tokio::spawn(process_file_events(event_rx, client.clone(), token, x));

    Ok(())
}

async fn process_file_events(
    mut event_rx: Receiver<FileMonitoring>,
    client: Arc<Client>,
    token: String,
    interval: u64,
) {
    let mut events = Vec::new();
    let mut seen_events = HashSet::new();

    loop {
        while let Ok(file_event) = event_rx.try_recv() {
            if seen_events.insert((file_event.file_path.clone(), file_event.event_type.clone())) {
                events.push(file_event);
            }
        }

        sleep(Duration::from_secs(interval)).await;

        if !events.is_empty() {
            let data = FileMonitoringPayload {
                file_monitoring: events.clone(),
            };

            match send_with_retries(&client, &data, &token, 3).await {
                Ok(_) => {
                    events.clear();
                    seen_events.clear();
                }
                Err(e) => {
                    println!("Failed to send events to API: {:?}", e);
                    log_error_fn(&format!("Failed to send events to API: {:?}", e));
                }
            }
        }
    }
}

async fn send_with_retries(
    client: &Client,
    data: &FileMonitoringPayload,
    token: &str,
    max_retries: usize,
) -> Result<(), anyhow::Error> {
    let mut retries = 0;
    let mut delay = Duration::from_secs(2);

    while retries < max_retries {
        match send_file_events_to_api(client, data, &token.to_string()).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                println!("Failed to send events to API: {:?}. Retrying...", e);
                sleep(delay).await;
                retries += 1;
                delay *= 2;
            }
        }
    }

    Err(anyhow::anyhow!(
        "Failed to send events after {} retries",
        max_retries
    ))
}
