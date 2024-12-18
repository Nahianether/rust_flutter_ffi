use ffi::{CstrToRust, RustToCstr};
use std::{ffi::c_char, time::Duration};

pub mod ffi;

pub mod api;
pub mod components;
pub mod models;
pub mod modules;

use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

use crate::api::token_from_employe_id::get_token_from_employee_id_fn;
// #[cfg(not(debug_assertions))]
// use crate::components::get_employee_id_from_app_name_fl::get_employee_id_fn;
use crate::models::tracforce_post_model::ActiveWindowWithTime;
use crate::models::tracforce_post_model::BrowserHistory;
use crate::models::tracforce_post_model::DesktopDetails;
use crate::models::tracforce_post_model::FileMonitoring;
use crate::models::tracforce_post_model::MouseKeyboardMovement;
use modules::check_and_update_application_fl::check_and_update_application;
use modules::components::uninstall_application::uninstall_application_fl::uninstall_app_fn;
use modules::start_application_mod::run_with_dynamic_config;

#[no_mangle]
pub extern "C" fn sum(a: usize, b: usize) -> usize {
    a + b
}

#[no_mangle]
pub extern "C" fn sum_async(a: usize, b: usize) -> usize {
    std::thread::sleep(Duration::from_secs(1));
    a + b
}

#[no_mangle]
pub extern "C" fn sum_str(a: *const c_char, b: *const c_char) -> *mut c_char {
    let av = a.to_native().parse::<usize>().unwrap();
    let bv = b.to_native().parse::<usize>().unwrap();
    let result = (av + bv).to_string();
    result.to_cstr()
}

lazy_static::lazy_static! {
    static ref CACHE: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    static ref SHARED_PAYLOAD: Arc<Mutex<String>> = Arc::new(Mutex::new(String::default()));
    static ref DESKTOP_MACADD_DATA: Arc<Mutex<String>> = Arc::new(Mutex::new(String::default()));
}

#[no_mangle]
pub extern "C" fn main_engine() {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    // #[cfg(not(debug_assertions))]
    // let employee_id = get_employee_id_fn();

    // #[cfg(debug_assertions)]
    let employee_id = "1859c1ae-88dc-4ce4-a825-c47d6faa2047".to_string();
    println!("Employee ID: {:?}", employee_id);

    let token_result = rt.block_on(
        // #[cfg(not(debug_assertions))]
        // get_token_from_employee_id_fn(&employee_id),
        // #[cfg(debug_assertions)]
        get_token_from_employee_id_fn("1859c1ae-88dc-4ce4-a825-c47d6faa2047"),
    );

    match token_result {
        Ok(token) => {
            println!("Token fetched successfully.");

            let mut shared_payload = SHARED_PAYLOAD.lock().unwrap();
            *shared_payload = token;

            if let Err(e) = rt.block_on(check_and_update_application(&employee_id)) {
                eprintln!("Failed to update application: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to get token from employee ID: {}", e);
            let _ = rt.block_on(uninstall_app_fn());
            return;
        }
    }

    rt.block_on(async move {
        run_with_dynamic_config().await;
    });
}
