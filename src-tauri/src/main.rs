#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
use app::command::minimize_image;

fn main() {
    tauri::Builder::default()
        .setup(app::setup::init)
        .invoke_handler(tauri::generate_handler![minimize_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
