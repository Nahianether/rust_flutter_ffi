use crate::models::tracforce_post_model::RunningProcess;
use crate::modules::components::log_error::log_error_fl::log_error_fn;
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use tokio::task;

pub async fn get_running_processes_fn() -> Vec<RunningProcess> {
    // Run the synchronous process-fetching code in a separate blocking task
    task::spawn_blocking(move || {
        let mut system = System::new_all();
        system.refresh_all();

        system
            .processes()
            .iter()
            .map(|(pid, process)| {
                let running_process = RunningProcess {
                    process_id: pid.as_u32(),
                    process_name: process.name().to_string(),
                    cpu_usage: process.cpu_usage(),
                };

                println!(
                    "Process ID: {}, Name: {}, CPU Usage: {}%",
                    running_process.process_id,
                    running_process.process_name,
                    running_process.cpu_usage
                );

                running_process
            })
            .collect()
    })
    .await
    .unwrap_or_else(|_| {
        log_error_fn("Failed to retrieve processes in async context.");
        Vec::new() // Return an empty vector if an error occurs
    })
}
