#[cfg(windows)]
use std::io;
#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;
#[cfg(unix)]
use std::process::Command;

#[cfg(windows)]
pub async fn disable_usb_storage() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let usb_stor = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Services\\USBSTOR",
        KEY_SET_VALUE,
    )?;

    // "Start" value 4 (disables the USB for storage)
    usb_stor.set_value("Start", &4u32)?;

    println!("USB storage has been disabled.");
    Ok(())
}

#[cfg(windows)]
pub async fn enable_usb_storage() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let usb_stor = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Services\\USBSTOR",
        KEY_SET_VALUE,
    )?;

    // "Start" value 3 (enables the USB for storage)
    usb_stor.set_value("Start", &3u32)?;

    println!("USB storage has been enabled.");
    Ok(())
}

#[cfg(target_os = "macos")]
pub async fn disable_usb_storage() -> Result<(), Box<dyn std::error::Error>> {
    // Unload the USB storage kernel extension
    Command::new("sudo")
        .arg("kextunload")
        .arg("-b")
        .arg("com.apple.iokit.IOUSBMassStorageClass")
        .output()?;

    println!("USB storage has been disabled on macOS.");
    Ok(())
}

#[cfg(target_os = "macos")]
pub async fn enable_usb_storage() -> Result<(), Box<dyn std::error::Error>> {
    // Load the USB storage kernel extension
    Command::new("sudo")
        .arg("kextload")
        .arg("-b")
        .arg("com.apple.iokit.IOUSBMassStorageClass")
        .output()?;

    println!("USB storage has been enabled on macOS.");
    Ok(())
}

#[cfg(target_os = "linux")]
pub async fn disable_usb_storage() -> Result<(), Box<dyn std::error::Error>> {
    // Unload the USB storage kernel module
    Command::new("sudo")
        .arg("modprobe")
        .arg("-r")
        .arg("usb-storage")
        .output()?;

    println!("USB storage has been disabled on Linux.");
    Ok(())
}

#[cfg(target_os = "linux")]
pub async fn enable_usb_storage() -> Result<(), Box<dyn std::error::Error>> {
    // Load the USB storage kernel module
    Command::new("sudo")
        .arg("modprobe")
        .arg("usb-storage")
        .output()?;

    println!("USB storage has been enabled on Linux.");
    Ok(())
}