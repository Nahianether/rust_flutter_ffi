use anyhow::Result;
use rdev::{listen, EventType};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

use crate::api::listen_mouse_keyboard_event_api_fl::send_event_data_to_api;
use crate::models::tracforce_post_model::MouseKeyboardMovement;
use crate::modules::components::log_error::log_error_fl::log_error_fn;

// pub async fn listen_to_keyboards_main_fn(x: u64, token: String) -> Result<()> {
//     let client = Client::builder()
//         .danger_accept_invalid_certs(true)
//         .connect_timeout(Duration::from_secs(15))
//         .timeout(Duration::from_secs(60))
//         .build()
//         .expect("Failed to create reqwest client");

//     let event_data = Arc::new(tokio::sync::Mutex::new(MouseKeyboardMovement {
//         x_position: 0.0,
//         y_position: 0.0,
//         key_pressed: String::new(),
//     }));

//     let (tx, mut rx) = mpsc::channel(100);

//     // Spawn task to listen for keyboard and mouse events
//     let event_data_clone = Arc::clone(&event_data);
//     tokio::spawn(async move {
//         if let Err(error) = listen(move |event| {
//             let tx = tx.clone();
//             tokio::spawn(async move {
//                 if let Err(e) = tx.send(event).await {
//                     log_error_fn(&format!("Failed to send event: {:?}", e));
//                 }
//             });
//         }) {
//             println!("Error: {:?}", error);
//             log_error_fn(&format!("Error during event listening: {:?}", error));
//         }
//     });

//     // Process events sequentially
//     tokio::spawn(async move {
//         while let Some(event) = rx.recv().await {
//             let mut data = event_data_clone.lock().await;
//             match event.event_type {
//                 EventType::KeyPress(key) => data.key_pressed = format!("{:?}", key),
//                 EventType::MouseMove { x, y } => {
//                     data.x_position = x as f64;
//                     data.y_position = y as f64;
//                 }
//                 _ => (),
//             }
//         }
//     });

//     // Send data to API every x seconds
//     loop {
//         sleep(Duration::from_secs(x)).await;

//         let data = event_data.lock().await.clone();
//         println!("Sending event data to API...: {:?}", data);

//         if let Err(e) = send_event_data_to_api(&client, &data, &token).await {
//             log_error_fn(&format!("Failed to send event data to API: {:?}", e));
//         } else {
//             let mut data = event_data.lock().await;
//             data.x_position = 0.0;
//             data.y_position = 0.0;
//             data.key_pressed = String::new();
//         }
//     }
// }

// ----------------------------------------------------------------------------------------------------

pub async fn listen_to_keyboards_main_fn(x: u64, token: String) -> Result<()> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    let event_data = Arc::new(tokio::sync::Mutex::new(MouseKeyboardMovement {
        key_pressed: Vec::new(),
    }));

    let (tx, mut rx) = mpsc::channel(100);

    let event_data_clone = Arc::clone(&event_data);
    tokio::spawn(async move {
        if let Err(error) = listen(move |event| {
            let tx = tx.clone();
            tokio::spawn(async move {
                if let Err(e) = tx.send(event).await {
                    log_error_fn(&format!("Failed to send event: {:?}", e));
                }
            });
        }) {
            println!("Error: {:?}", error);
            log_error_fn(&format!("Error during event listening: {:?}", error));
        }
    });

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            let mut event_data = event_data_clone.lock().await;

            match event.event_type {
                EventType::KeyPress(key) => {
                    event_data
                        .key_pressed
                        .push(format!("{:?}", key).replace("Key", ""));
                }
                _ => (),
            }
        }
    });

    loop {
        sleep(Duration::from_secs(x)).await;

        let mut event_data = event_data.lock().await;

        if !event_data.key_pressed.is_empty() {
            if let Err(e) = send_event_data_to_api(&client, &*event_data, &token).await {
                log_error_fn(&format!("Failed to send event data to API: {:?}", e));
            } else {
                println!("Event data sent successfully.");

                event_data.key_pressed.clear();
            }
        }
    }
}

// output

// {
//     "x_position": [100.0, 150.0, 200.0],
//     "y_position": [300.0, 350.0, 400.0],
//     "key_pressed": ["KeyA", "KeyB", "KeyC"]
// }

// ----------------------------------------------------------------------------------------------------

// pub async fn listen_to_keyboards_main_fn(x: u64, token: String) -> Result<()> {
//     let client = Client::builder()
//         .danger_accept_invalid_certs(true)
//         .connect_timeout(Duration::from_secs(15))
//         .timeout(Duration::from_secs(60))
//         .build()
//         .expect("Failed to create reqwest client");

//     let event_data = Arc::new(tokio::sync::Mutex::new(MouseKeyboardMovement {
//         key_pressed: Vec::new(),
//     }));

//     let (tx, mut rx) = mpsc::channel(100);

//     let event_data_clone = Arc::clone(&event_data);
//     tokio::spawn(async move {
//         if let Err(error) = listen(move |event| {
//             let tx = tx.clone();
//             tokio::spawn(async move {
//                 if let Err(e) = tx.send(event).await {
//                     log_error_fn(&format!("Failed to send event: {:?}", e));
//                 }
//             });
//         }) {
//             println!("Error: {:?}", error);
//             log_error_fn(&format!("Error during event listening: {:?}", error));
//         }
//     });

//     tokio::spawn(async move {
//         while let Some(event) = rx.recv().await {
//             let mut event_data = event_data_clone.lock().await;

//             match event.event_type {
//                 EventType::KeyPress(key) => {
//                     // let key_str = format!("{:?}", key);
//                     let key_str = format!("{:?}", key).replace("Key", "");

//                     if key_str == "Enter" || key_str == "Space" {
//                         event_data.key_pressed.push(MouseKeyboardMovementSingle {
//                             key_pressed: Vec::new(),
//                         });
//                     } else {
//                         if event_data.key_pressed.is_empty() {
//                             event_data.key_pressed.push(MouseKeyboardMovementSingle {
//                                 key_pressed: Vec::new(),
//                             });
//                         }
//                         if let Some(last_group) = event_data.key_pressed.last_mut() {
//                             last_group.key_pressed.push(key_str);
//                         }
//                     }
//                 }
//                 _ => (),
//             }
//         }
//     });

//     loop {
//         sleep(Duration::from_secs(x)).await;

//         let mut event_data = event_data.lock().await;

//         if !event_data.key_pressed.is_empty() {
//             println!("Sending event data to API...: {:?}", *event_data);

//             if let Err(e) = send_event_data_to_api(&client, &*event_data, &token).await {
//                 log_error_fn(&format!("Failed to send event data to API: {:?}", e));
//             } else {
//                 println!("Event data sent successfully.");

//                 event_data.key_pressed.clear();
//             }
//         }
//     }
// }

// output
// {
//   key_pressed:
//   [
//     {
//       key_pressed:
//       ["H", "O", "W"]

//     },
//     {
//       key_pressed:
//       ["A", "R", "E"]

//     }
//   ]
// }
