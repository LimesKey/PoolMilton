#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rsntp::SntpClient;
use chrono::{DateTime, Local};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![display_time])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn display_time() -> String {
    let client = SntpClient::new();
    let result = client.synchronize("pool.ntp.org").unwrap();
    let local_time: DateTime<Local> = DateTime::from(result.datetime().into_chrono_datetime().unwrap());
    local_time.date_naive().to_string()
}
