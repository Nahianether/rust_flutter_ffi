use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

#[cfg(windows)]
const HOSTS_FILE_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts";

#[cfg(target_os = "macos")]
const HOSTS_FILE_PATH: &str = "/etc/hosts";

#[cfg(target_os = "linux")]
const HOSTS_FILE_PATH: &str = "/etc/hosts";

pub async fn block_website(websites: Vec<&str>) -> io::Result<()> {
    let hosts_path = Path::new(HOSTS_FILE_PATH);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&hosts_path)?;

    let content = std::fs::read_to_string(&hosts_path)?;
    for website in &websites {
        if content.contains(website) {
            println!("Website '{}' is already blocked.", website);
            return Ok(());
        }
    }
    for website in websites {
        writeln!(
            file,
            "127.0.0.1 {}\n127.0.0.1 www.{}\n127.0.0.1 https://www.{}",
            website, website, website
        )?;
        println!("Website '{}' has been blocked.", website);
    }
    Ok(())
}

pub async fn unblock_website(websites: Vec<&str>) -> io::Result<()> {
    let hosts_path = Path::new(HOSTS_FILE_PATH);
    let content = std::fs::read_to_string(&hosts_path)?;
    let mut new_content = String::new();

    for line in content.lines() {
        let mut should_keep = true;
        for website in &websites {
            if line.contains(website) {
                should_keep = false;
                break;
            }
        }
        if should_keep {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }

    std::fs::write(&hosts_path, new_content)?;

    for website in websites {
        println!("Website '{}' has been unblocked.", website);
    }
    Ok(())
}
