// use std::env;

use anyhow::Result;
use std::fs;

use crate::{
    api::{
        check_application_version_fl::check_application_version_from_api,
        download_application_api_fl::download_application_from_api_fn,
    },
    components::{application_version::VERSION, generate_update_script_fl::generate_update_script},
    modules::components::log_error::log_error_fl::log_error_fn,
};

pub async fn check_and_update_application(employee_id: &String) -> Result<()> {
    let client = reqwest::Client::new();
    println!("{:?}", client);

    let server_version = match check_application_version_from_api().await {
        Ok(version) => version.to_string(),
        Err(e) => {
            eprintln!("Application Version Check Failed: {:?}", e);
            log_error_fn(&format!("Application Version Check Failed: {:?}", e));
            String::new() // Return an empty string on error
        }
    };

    let current_version = VERSION.to_string();
    println!("Application Version: {}", current_version);

    if server_version <= current_version {
        println!("Your application is up-to-date.");
        return Ok(());
    }

    println!(
        "New version available: {} (current: {}). Updating...",
        server_version, current_version
    );

    // match download_application_from_api_fn(&employee_id).await {
    //     Ok(_) => (),
    //     Err(e) => {
    //         eprintln!("Download Application Failed: {:?}", e);
    //         log_error_fn(&format!("Download Application Failed: {:?}", e));
    //         return Err(e.into());
    //     }
    // };
    let mut attempts = 0;
    while attempts < 3 {
        match download_application_from_api_fn(&employee_id).await {
            Ok(_) => break,
            Err(e) => {
                attempts += 1;
                eprintln!(
                    "Download Application Failed (attempt {}): {:?}",
                    attempts, e
                );
                if attempts < 3 {
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                } else {
                    log_error_fn(&format!("Download Application Failed: {:?}", e));
                    return Err(e.into());
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    let app_name = format!("{}_track_force_rs.exe", employee_id);
    #[cfg(target_os = "macos")]
    let app_name = format!("{}_track_force_rs", employee_id);

    // source_path from where the new executable will be copied
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))?
        .to_path_buf();

    // Create the new_app directory inside the executable directory
    let new_app_dir = exe_dir.join("new_app");
    if !new_app_dir.exists() {
        fs::create_dir_all(&new_app_dir)?;
    }

    // Assuming the downloaded exe is stored in the new_app directory
    let source_path_buf = new_app_dir.join(format!("{}_track_force_rs.exe", employee_id));
    let source_path = source_path_buf
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to convert source path to string"))?;

    // destination_path where the new executable will be pasted
    let destination_path_buf = exe_dir.join(&app_name);
    let destination_path = destination_path_buf
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to convert destination path to string"))?;

    // start_path to run the new executable
    let start_path = destination_path;

    println!("Source Path: {}", source_path);
    println!("Destination Path: {}", destination_path);
    println!("Start Path: {}", start_path);

    let script_path =
        generate_update_script(app_name.as_str(), source_path, destination_path, start_path)?;

    println!("Script generated at: {}", script_path);

    let status = std::process::Command::new("cmd")
        .arg("/C")
        .arg(script_path)
        .status()?;

    if !status.success() {
        eprintln!("Script execution failed with code: {:?}", status.code());
    } else {
        println!("Update script executed successfully.");
    }

    // println!("Update completed successfully.");
    Ok(())
}
