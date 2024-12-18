#[cfg(not(debug_assertions))]
use crate::components::get_employee_id_from_app_name_fl::get_employee_id_fn;
use crate::modules::components::log_error::log_error_fl::log_error_fn;
use sysinfo::{ProcessExt, System, SystemExt};
use tokio::time::{sleep, Duration};

const RETRY_ATTEMPTS: u8 = 3;
const RETRY_DELAY: Duration = Duration::from_secs(2);

pub async fn is_app_running() -> bool {
    for attempt in 1..=RETRY_ATTEMPTS {
        let running = tokio::task::spawn_blocking(|| {
            let system = System::new_all();
            #[cfg(not(debug_assertions))]
            let employee_id = get_employee_id_fn();
            #[cfg(debug_assertions)]
            let employee_id = "fee5cecd-3472-42e4-b02b-6b7aa50f1ff9".to_string();
            let app_name = format!("{}_track_force_rs", employee_id);
            // let app_name = "track_force_rs".to_string();
            system
                .processes()
                .values()
                .any(|process| process.name().contains(&app_name))
        })
        .await
        .unwrap_or(false);

        if running {
            println!("App is running.");
            return true;
        } else {
            log_error_fn(&format!("Attempt {}: App is not running.", attempt));
            if attempt < RETRY_ATTEMPTS {
                sleep(RETRY_DELAY).await;
            }
        }
    }
    false
}
