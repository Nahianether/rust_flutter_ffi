#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate winreg;

use anyhow::Result;
use std::env;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub async fn delete_executable() -> Result<(), String> {
    let exe_path = env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {:?}", e))?;
    let exe_dir = exe_path
        .parent()
        .ok_or("Failed to get executable directory")?;

    #[cfg(windows)]
    {
        let batch_script_path = exe_dir.join("delete_exe.bat");
        let mut batch_script = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&batch_script_path)
            .await
            .map_err(|e| format!("Failed to create batch script: {:?}", e))?;

        batch_script
            .write_all(
                format!(
                    "@echo off\n\
                    timeout /t 5 /nobreak > nul\n\
                    del \"{}\"\n\
                    del \"%~f0\"\n",
                    exe_path.to_string_lossy()
                )
                .as_bytes(),
            )
            .await
            .map_err(|e| format!("Failed to write to batch script: {:?}", e))?;

        Command::new("cmd")
            .args(&["/C", batch_script_path.to_string_lossy().as_ref()])
            .spawn()
            .map_err(|e| format!("Failed to execute batch script: {:?}", e))?;
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let shell_script_path = exe_dir.join("delete_exe.sh");
        let mut shell_script = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&shell_script_path)
            .await
            .map_err(|e| format!("Failed to create shell script: {:?}", e))?;

        shell_script
            .write_all(
                format!(
                    "#!/bin/bash\n\
                    sleep 5\n\
                    rm \"{}\"\n\
                    rm -- \"$0\"\n",
                    exe_path.to_string_lossy()
                )
                .as_bytes(),
            )
            .await
            .map_err(|e| format!("Failed to write to shell script: {:?}", e))?;

        Command::new("sh")
            .arg(shell_script_path.to_string_lossy().as_ref())
            .spawn()
            .map_err(|e| format!("Failed to execute shell script: {:?}", e))?;
    }

    Ok(())
}
