#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rsntp::SntpClient;
use chrono::{DateTime, Local};

fn main() {
    // initialize the custom invoke system as a HTTP server, allowing the given origins to access it.
    let http = tauri_invoke_http::Invoke::new(if cfg!(feature = "custom-protocol") {
      ["tauri://localhost"]
    } else {
      ["http://localhost:5173"]
    });
    tauri::Builder::default()
      .invoke_system(http.initialization_script(), http.responder())
      .setup(move |app| {
        http.start(app.handle());
        Ok(())
      })
      .invoke_handler(tauri::generate_handler![display_time])
      .run(tauri::generate_context!())
      .expect("error while running tauri application")
}

#[tauri::command]
fn display_time() -> String {
    let client = SntpClient::new();
    let result = client.synchronize("pool.ntp.org").unwrap();
    let local_time: DateTime<Local> = DateTime::from(result.datetime().into_chrono_datetime().unwrap());
    local_time.date_naive().to_string()
}
