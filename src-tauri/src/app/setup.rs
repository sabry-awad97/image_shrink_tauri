use std::error::Error;
use tauri::{App, WindowBuilder};

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    let handle = app.handle();
    let window_builder =
        WindowBuilder::new(&handle, "main", tauri::WindowUrl::App("index.html".into()))
            .title("ImageShrink")
            .inner_size(800.0, 600.0)
            .fullscreen(false)
            .resizable(true);

    let _window = window_builder.build()?;
    Ok(())
}
