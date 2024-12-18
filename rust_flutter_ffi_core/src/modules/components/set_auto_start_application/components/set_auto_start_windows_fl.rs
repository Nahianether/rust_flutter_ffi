#[cfg(windows)]
use crate::modules::components::log_error::log_error_fl::log_error_fn;
#[cfg(windows)]
use tokio::time::{sleep, Duration};

#[cfg(windows)]
pub async fn set_auto_start_windows() {
    extern crate winapi;
    extern crate winreg;

    use std::env;
    use winreg::enums::*;
    use winreg::RegKey;

    #[cfg(not(debug_assertions))]
    use crate::components::get_employee_id_from_app_name_fl::get_employee_id_fn;

    let exe_path = match env::current_exe() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(e) => {
            log_error_fn(&format!("Failed to get executable path: {}", e));
            return;
        }
    };

    for attempt in 1..=3 {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        // let conn = Connection::open("employee_id.db").expect("Failed to open SQLite database");
        // let employee_id = match get_employee_id_from_db(&conn) {
        //     Ok(Some(id)) => id,
        //     Ok(None) => {
        //         log_error_fn("Employee ID not found in the database.");
        //         return;
        //     }
        //     Err(e) => {
        //         log_error_fn(&format!("Failed to get employee ID from database: {:?}", e));
        //         return;
        //     }
        // };
        #[cfg(not(debug_assertions))]
        let employee_id = get_employee_id_fn();
        #[cfg(debug_assertions)]
        let employee_id = "fee5cecd-3472-42e4-b02b-6b7aa50f1ff9".to_string();
        let app_name = format!("{}_track_force_rs", employee_id);
        // let app_name = "track_force_rs".to_string();

        if let Ok((key, _disp)) =
            hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
        {
            if key
                .set_value(app_name, &exe_path)
                .is_ok()
            {
                println!("Auto-start successfully set for Windows.");
                return;
            }
        }

        println!(
            "Attempt {} to set auto-start failed on Windows. Retrying...",
            attempt
        );
        log_error_fn(&format!(
            "Attempt {} to set auto-start failed on Windows.",
            attempt
        ));

        if attempt < 3 {
            sleep(Duration::from_secs(2)).await;
        } else {
            println!("Failed to set auto-start on Windows after 3 attempts.");
            log_error_fn("Failed to set auto-start on Windows after 3 attempts.");
        }
    }
}
