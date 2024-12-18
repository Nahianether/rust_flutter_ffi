use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::Write;

pub fn generate_update_script(
    app_name: &str,
    source_path: &str,
    destination_path: &str,
    start_path: &str,
) -> Result<String> {
    let exe_dir = env::current_exe()?
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))?
        .to_path_buf();

    let script_path = exe_dir.join("update_application.bat");
    let script_path_str = script_path.to_str().unwrap_or_default();

    let script_content = format!(
        r#"@echo off
REM Uninstall the current application
echo Uninstalling the current application...
taskkill /IM "{app_name}" /F >nul 2>&1
timeout /t 2 >nul

REM Copy the new executable to a temporary location
set TEMP_COPY="{destination_path}.tmp"
echo Copying the new executable to temporary location...
copy "{source_path}" %TEMP_COPY% /Y
if errorlevel 1 (
    echo Failed to copy the new executable to temporary location. Exiting.
    exit /b 1
)

REM Replace the current executable with the new version
echo Replacing the current executable with the new version...
move /Y %TEMP_COPY% "{destination_path}"
if errorlevel 1 (
    echo Failed to move the new executable to the destination. Exiting.
    exit /b 1
)

REM Run the new executable
echo Starting the new application...
start "" "{start_path}"

REM Close this console
exit
"#,
        app_name = app_name,
        source_path = source_path,
        destination_path = destination_path,
        start_path = start_path
    );

    let mut file = File::create(&script_path)?;
    file.write_all(script_content.as_bytes())?;

    println!("Update script generated at: {}", script_path_str);

    Ok(script_path_str.to_string())
}
