// use std::io::{self};
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;
// use sysinfo::{Pid, ProcessExt, System, SystemExt};

// static BLOCKING_ACTIVE: once_cell::sync::Lazy<Arc<Mutex<bool>>> =
//     once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(false)));

// pub async fn start_blocking_application(process_names: Vec<String>) -> io::Result<()> {
//     let blocking_flag = Arc::clone(&BLOCKING_ACTIVE);
//     *blocking_flag.lock().unwrap() = true;

//     let process_names = Arc::new(process_names);

//     // Spawn a new thread to block the applications
//     thread::spawn(move || {
//         let mut system = System::new_all();

//         println!("Blocking the following applications: {:?}", process_names);

//         while *blocking_flag.lock().unwrap() {
//             system.refresh_all();

//             // Iterate over each process name in the list
//             for process_name in process_names.iter() {
//                 let mut found = false;

//                 // Check and kill the target processes if they are running
//                 for (pid, process) in system.processes() {
//                     if process.name().eq_ignore_ascii_case(process_name) {
//                         println!("Killing process: {} (PID: {})", process.name(), pid);
//                         process.kill();
//                         found = true;
//                     }
//                 }

//                 if !found {
//                     println!("Process not found: {}", process_name);
//                 }
//             }

//             thread::sleep(Duration::from_secs(5));
//         }

//         println!("Application blocking stopped for {:?}", process_names);
//     });
//     Ok(())
// }

// pub async fn stop_blocking_application() -> io::Result<()> {
//     let blocking_flag = Arc::clone(&BLOCKING_ACTIVE);

//     *blocking_flag.lock().unwrap() = false;
//     println!("Unblocking all applications.");

//     Ok(())
// }


// --------------------------------------------------------------------------------------------------------------

use std::io::{self};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{Pid, ProcessExt, System, SystemExt};
#[cfg(unix)]
use sysinfo::PidExt;

static BLOCKING_ACTIVE: once_cell::sync::Lazy<Arc<Mutex<bool>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(false)));

/// Starts blocking the applications by terminating their processes if they are running.
pub async fn start_blocking_applications(process_names: Vec<String>) -> io::Result<()> {
    let blocking_flag = Arc::clone(&BLOCKING_ACTIVE);
    *blocking_flag.lock().unwrap() = true;

    let process_names = Arc::new(process_names);

    // Spawn a new thread to block the applications
    thread::spawn(move || {
        let mut system = System::new_all();

        println!("Blocking the following applications: {:?}", process_names);

        while *blocking_flag.lock().unwrap() {
            system.refresh_all();

            // Iterate over each process name in the list
            for process_name in process_names.iter() {
                let mut found = false;

                // Check and kill the target processes if they are running
                for (pid, process) in system.processes() {
                    if process.name().eq_ignore_ascii_case(process_name) {
                        println!("Killing process: {} (PID: {})", process.name(), pid);
                        kill_process(*pid, process.name());
                        found = true;
                    }
                }

                if !found {
                    println!("Process not found: {}", process_name);
                }
            }

            thread::sleep(Duration::from_secs(5));
        }

        println!("Application blocking stopped for {:?}", process_names);
    });

    Ok(())
}

pub async fn stop_blocking_applications() -> io::Result<()> {
    let blocking_flag = Arc::clone(&BLOCKING_ACTIVE);

    *blocking_flag.lock().unwrap() = false;
    println!("Unblocking all applications.");

    Ok(())
}

fn kill_process(pid: Pid, process_name: &str) {
    #[cfg(windows)]
    {
        // On Windows, use sysinfo's `kill` method
        println!("Killing process on Windows: {}", process_name);
        let system = System::new_all();
        if let Some(process) = system.process(pid) {
            process.kill();
        }
    }

    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;

        println!("Killing process on Unix: {}", process_name);
        // Convert `sysinfo::Pid` (usize) to `nix::unistd::Pid` (i32)
        if let Ok(nix_pid) = i32::try_from(pid.as_u32()) {
            if let Err(e) = kill(Pid::from_raw(nix_pid), Signal::SIGKILL) {
                eprintln!("Failed to kill process {}: {}", process_name, e);
            }
        } else {
            eprintln!("Invalid PID for process {}: {}", process_name, pid);
        }
    }
}

