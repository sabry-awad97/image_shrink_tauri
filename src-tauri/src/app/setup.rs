use tauri::App;

use super::window;

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    window::create_main_window(&handle);
    Ok(())
}
