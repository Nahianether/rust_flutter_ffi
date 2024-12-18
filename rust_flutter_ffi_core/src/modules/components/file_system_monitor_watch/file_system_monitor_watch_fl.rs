use crate::api::send_file_events_to_api_fl::send_file_events_to_api;
use crate::models::tracforce_post_model::FileMonitoringPayload;
use crate::modules::components::log_error::log_error_fl::log_error_fn;
use crate::FileMonitoring;
use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
use reqwest::Client;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
pub async fn file_system_monitor_fn(
    x: u64,
    token: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");
    let (tx, mut rx) = mpsc::channel(100);
    let buffer: Arc<tokio::sync::Mutex<Vec<FileMonitoring>>> =
        Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let collect_duration = Duration::from_secs(x);
    // Define the path to watch based on the OS
    let watch_path = if cfg!(target_os = "windows") {
        PathBuf::from("D:/")
    } else if cfg!(target_os = "macos") {
        let home_dir = env::var("HOME").expect("Failed to get home directory");
        Path::new(&home_dir).join("Downloads")
    } else {
        panic!("Unsupported platform");
    };
    println!("Monitoring path: {:?}", watch_path);
    // Create file watcher
    let mut watcher = recommended_watcher(move |res| match tx.try_send(res) {
        Ok(_) => {}
        Err(e) => log_error_fn(&format!("Failed to send file event: {:?}", e)),
    })
    .map_err(|e| {
        log_error_fn(&format!("Failed to create watcher: {:?}", e));
        e
    })?;
    watcher.watch(&watch_path, RecursiveMode::Recursive)?;
    // Main async loop to process file events and periodically send them to the API
    let buffer_ref = Arc::clone(&buffer);
    tokio::spawn(async move {
        loop {
            // Check if there are any events
            while let Some(event) = rx.recv().await {
                match event {
                    Ok(Event { kind, paths, .. }) => {
                        for path in paths {
                            let event_name = match kind {
                                notify::event::EventKind::Create(_) => "File created".to_string(),
                                notify::event::EventKind::Remove(_) => "File deleted".to_string(),
                                notify::event::EventKind::Modify(_) => "File modified".to_string(),
                                notify::event::EventKind::Access(_) => "File accessed".to_string(),
                                _ => continue, // Skip other events
                            };
                            let file_name = path.to_string_lossy().into_owned();
                            let mut buffer_lock = buffer_ref.lock().await;
                            buffer_lock.push(FileMonitoring {
                                event_type: event_name.clone(),
                                file_path: file_name.clone(),
                                timestamp: chrono::Utc::now().to_rfc3339(),
                            });
                            println!(
                                "Collected file event: {}, File name: {:?}",
                                event_name, file_name
                            );
                        }
                    }
                    Err(e) => log_error_fn(&format!("Watch error: {:?}", e)),
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
    });
    // Task to periodically send buffered events to API
    let buffer_ref_for_sending = Arc::clone(&buffer);
    let client_for_sending = client.clone();
    let token_for_sending = token.clone();
    tokio::spawn(async move {
        loop {
            sleep(collect_duration).await; // Wait for the specified duration
                                           // Send data to API if there are events in the buffer
            let mut buffer_lock = buffer_ref_for_sending.lock().await;
            if !buffer_lock.is_empty() {
                let events = buffer_lock.clone();
                let client = client_for_sending.clone();
                let token = token_for_sending.clone();
                let data = FileMonitoringPayload {
                    file_monitoring: events.clone(),
                };
                if let Err(e) = send_file_events_to_api(&client, &data, &token).await {
                    log_error_fn(&format!("Failed to send file events to API: {:?}", e));
                } else {
                    println!("File system events sent to API.");
                    buffer_lock.clear(); // Clear buffer after successful sending
                }
            } else {
                buffer_lock.clear(); // Clear buffer if there are no events
                println!("No file system events to send.");
            }
        }
    });
    Ok(())
}
