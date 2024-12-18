use regex::Regex;
use std::env;

/// Gets the employee ID from the application name or DMG file name (depending on the OS).
pub fn get_employee_id_fn() -> String {
    #[cfg(target_os = "windows")]
    let employee_id = get_employee_id_from_exe_name()
        .expect("Failed to extract employee ID from executable name");

    #[cfg(target_os = "macos")]
    let employee_id =
        get_employee_id_from_dmg_name().expect("Failed to extract employee ID from DMG name");

    employee_id
}

#[cfg(target_os = "windows")]
fn get_employee_id_from_exe_name() -> Option<String> {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let app_name = exe_path.file_stem()?.to_string_lossy();

    let re =
        Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
            .unwrap();

    if let Some(captures) = re.captures(&app_name) {
        if let Some(id_match) = captures.get(0) {
            return Some(id_match.as_str().to_string());
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn get_employee_id_from_dmg_name() -> Option<String> {
    if let Ok(exe_path) = env::current_exe() {
        if let Some(dmg_name) = exe_path.file_name() {
            let dmg_name_str = dmg_name.to_string_lossy();
            let re = Regex::new(
                r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}",
            )
            .unwrap();

            if let Some(captures) = re.captures(&dmg_name_str) {
                if let Some(id_match) = captures.get(0) {
                    return Some(id_match.as_str().to_string());
                }
            }
        }
    }
    None
}

// pub fn get_employee_id_from_db(conn: &Connection) -> Result<Option<String>> {
//     let mut stmt = conn.prepare("SELECT id FROM employee_info")?;
//     let employee_id_iter = stmt.query_map([], |row| row.get(0))?;

//     for employee_id in employee_id_iter {
//         return Ok(Some(employee_id?));
//     }

//     Ok(None)
// }

// fn store_employee_id_in_db(conn: &Connection, employee_id: &str) -> Result<()> {
//     conn.execute(
//         "INSERT OR REPLACE INTO employee_info (id) VALUES (?1)",
//         &[employee_id],
//     )?;
//     Ok(())
// }
