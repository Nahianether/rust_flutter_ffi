#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate winreg;

use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;

pub fn log_error_fn(message: &str) {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");

    let log_dir = exe_dir.join("log");
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir).expect("Failed to create log directory");
    }

    let log_file_path = log_dir.join("track_force_rs.log");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("Failed to open log file");

    writeln!(file, "{}", message).expect("Failed to write to log file");
}
