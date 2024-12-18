#[cfg(unix)]
use std::env;

// #[cfg(target_os = "windows")]
// fn is_admin() -> bool {
//     use windows::Win32::Security::{CheckTokenMembership, AllocateAndInitializeSid, SECURITY_NT_AUTHORITY, WinBuiltinAdministratorsSid};
//     use windows::Win32::Foundation::{PSID, BOOL};
//     use windows::Win32::System::SystemServices::TOKEN_QUERY;
//     use windows::Win32::System::Threading::OpenProcessToken;
//     use windows::Win32::System::Threading::GetCurrentProcess;

//     unsafe {
//         let mut token_handle = std::ptr::null_mut();
//         if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).0 != 0 {
//             let mut admin_group: PSID = std::ptr::null_mut();
//             let authority = SECURITY_NT_AUTHORITY;
//             if AllocateAndInitializeSid(
//                 &authority,
//                 2,
//                 WinBuiltinAdministratorsSid,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 0,
//                 &mut admin_group,
//             )
//             .as_bool()
//             {
//                 let mut is_admin: BOOL = BOOL(0);
//                 let result = CheckTokenMembership(token_handle, admin_group, &mut is_admin);
//                 return result.as_bool() && is_admin.0 != 0;
//             }
//         }
//     }
//     false
// }

#[cfg(target_os = "linux")]
fn is_admin() -> bool {
    // Check if the current user ID is 0 (root)
    match env::var("USER") {
        Ok(user) => user == "root",
        Err(_) => false,
    }
}

#[cfg(target_os = "macos")]
pub fn is_admin() -> bool {
    // macOS uses the same logic as Linux since it's UNIX-based
    match env::var("USER") {
        Ok(user) => user == "root",
        Err(_) => false,
    }
}