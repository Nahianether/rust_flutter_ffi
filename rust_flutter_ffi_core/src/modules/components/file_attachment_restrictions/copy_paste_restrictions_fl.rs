#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
use std::ptr;
#[cfg(windows)]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(windows)]
use std::thread;

#[cfg(target_os = "windows")]
use winapi::um::winuser::{OpenClipboard, CloseClipboard, EmptyClipboard, GetClipboardSequenceNumber};

// Global atomic flag to enable/disable clipboard blocking
#[cfg(windows)]
static IS_BLOCKING: AtomicBool = AtomicBool::new(true);

/// Function to monitor and clear clipboard content
#[cfg(windows)]
pub fn block_copy_paste() {
    thread::spawn(|| {
        let mut last_seq = unsafe { GetClipboardSequenceNumber() };
        let mut last_error_logged = false;

        loop {
            // Check if blocking is active
            if !IS_BLOCKING.load(Ordering::SeqCst) {
                println!("Clipboard blocking disabled.");
                break;
            }

            // Get the current clipboard sequence number
            let current_seq = unsafe { GetClipboardSequenceNumber() };

            if current_seq != last_seq {
                println!("Clipboard content detected. Clearing clipboard.");
                
                // Open and clear the clipboard
                let opened = unsafe { OpenClipboard(ptr::null_mut()) };

                if opened != 0 {
                    unsafe {
                        EmptyClipboard();
                        CloseClipboard();
                    }
                    println!("Clipboard cleared successfully.");
                    last_error_logged = false; // Reset error flag after success
                } else {
                    if !last_error_logged {
                        println!("Failed to open clipboard. Another process might be using it.");
                        last_error_logged = true; // Log error only once
                    }
                }

                // Update the last sequence number
                last_seq = current_seq;
            }

            // Sleep for a short interval before checking again
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

/// Function to unblock copy-paste
#[cfg(windows)]
pub fn unblock_copy_paste() {
    IS_BLOCKING.store(false, Ordering::SeqCst);
    println!("Copy-paste functionality unblocked.");
}