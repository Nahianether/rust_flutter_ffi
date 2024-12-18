use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DesktopDetails {
    pub host_name: String,
    pub ip_address: String,
    pub os_name: String,
    pub os_version: String,
    pub ram_size: u64,
    pub ram_usage: u64,
    pub mac_add: String,
    pub cpu_usage: f32,
    pub app_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScreenshotImage {
    pub image_path: String,
    pub image_name: String,
}

// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// pub struct MouseKeyboardMovementSingle {
//     pub key_pressed: Vec<String>,
// }

// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// pub struct MouseKeyboardMovement {
//     pub key_pressed: Vec<MouseKeyboardMovementSingle>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MouseKeyboardMovement {
    pub key_pressed: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunningProcess {
    pub process_name: String,
    pub process_id: u32,
    pub cpu_usage: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextFromImage {
    pub image_name: String,
    pub detected_domain: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrowserHistory {
    pub url: Vec<BrowserHistorySingle>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrowserHistorySingle {
    pub url: String,
    pub title: String,
    pub last_visit_time: String,
    pub visit_count: i32,
    pub visit_duration: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveWindowWithTime {
    pub window_name: String,
    pub time_spent: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMonitoring {
    pub event_type: String,
    pub file_path: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMonitoringPayload {
    pub file_monitoring: Vec<FileMonitoring>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ApiModel {
    pub int_tracking_type_id: i32,
    pub str_tracking_type: String,
    pub str_token: String,
    pub str_json_body: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EmployeeInfo {
    pub str_employee_id: String,
    pub str_name: String,
    pub int_account_id: i32,
    pub str_token: String,
    pub is_active: bool,
    pub dte_last_login: Option<String>,
    pub dte_created_at: String,
    pub is_activation: bool,
    pub dte_activation_date: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub capture_interval: u64,
    pub chrome_history_interval: u64,
    pub active_window_interval: u64,
    pub keyboard_listener_interval: u64,
    pub log_send_interval: u64,
    pub uninstall_after_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigFile {
    #[serde(rename = "appBlockList")]
    pub is_application_block_data: Option<Vec<String>>,
    #[serde(rename = "webBlockList")]
    pub is_website_block_data: Option<Vec<String>>,
    #[serde(rename = "isUsbBlock")]
    pub is_usb_block: Option<bool>,
    #[serde(rename = "ipAddressBlockList")]
    pub is_ip_address_block_data: Option<Vec<String>>,
    #[serde(rename = "isUninstalled")]
    pub is_application_uninstall: Option<bool>,
    #[serde(rename = "isApplicationBlock")]
    pub is_application_block: Option<bool>,
    #[serde(rename = "isWebBlock")]
    pub is_website_block: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ErrorLogSingle {
    pub error_log: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ErrorLog {
    pub error_logs: Vec<ErrorLogSingle>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct EmailDetails {
    pub body: String,
    pub image_path: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MeetingParticipant {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MeetingDetails {
    pub participant: Vec<MeetingParticipant>,
    pub image_path: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SearchKeyFromUrl {
    pub search_key: Option<String>,
    pub url: String,
    pub date_time: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SearchKeyFromUrlList {
    pub search_key_from_url: Vec<SearchKeyFromUrl>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SearchKeyFromUrlPayload {
    pub url: String,
    pub date_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SocialMediaDetect {
    pub image_path: String,
    pub social_media: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OnlineMessageDetect {
    pub image_path: String,
    pub message: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConsoleCapture {
    pub image_path: String,
    pub body: String,
}
