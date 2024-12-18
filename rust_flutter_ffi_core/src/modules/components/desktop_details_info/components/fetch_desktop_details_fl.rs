use crate::{components::application_version::VERSION, DesktopDetails, DESKTOP_MACADD_DATA};
use anyhow::{Error, Result};
use hostname::get;
use if_addrs::get_if_addrs;
use std::{
    net::IpAddr,
    sync::{Arc, Mutex},
};
use sysinfo::{CpuExt, System, SystemExt};

pub async fn fetch_desktop_details() -> Result<DesktopDetails, Error> {
    let mac_address_data = Arc::new(Mutex::new(DESKTOP_MACADD_DATA.lock().unwrap().clone()));

    let hostname = get()
        .map_err(|e| anyhow::anyhow!("Failed to get hostname: {:?}", e))?
        .into_string()
        .unwrap();

    let ip_addresses: Vec<IpAddr> = get_if_addrs()?
        .into_iter()
        .filter_map(|iface| Some(iface.addr.ip()))
        .collect();

    let sys_info = tokio::task::spawn_blocking(move || {
        let mut sys = System::new_all();
        sys.refresh_all();
        (
            sys.name().unwrap_or_else(|| "Unknown".to_string()),
            sys.os_version().unwrap_or_else(|| "Unknown".to_string()),
            sys.total_memory(),
            sys.used_memory(),
            sys.global_cpu_info().cpu_usage(),
        )
    })
    .await?;

    let (os_name, os_version, total_memory, used_memory, cpu_usage) = sys_info;

    let mac_add = mac_address_data.lock().unwrap().clone();

    let app_version = VERSION.to_string();

    Ok(DesktopDetails {
        host_name: hostname,
        ip_address: ip_addresses
            .iter()
            .map(|ip| ip.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        os_name,
        os_version,
        ram_size: total_memory,
        ram_usage: used_memory,
        mac_add,
        cpu_usage,
        app_version,
    })
}
