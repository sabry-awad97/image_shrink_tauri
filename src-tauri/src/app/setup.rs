use std::error::Error;
use tauri::{App, Manager, WindowBuilder};

use super::menu::MainMenu;

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    let main_menu = MainMenu::new();

    let handle = app.handle();
    let mut window_builder =
        WindowBuilder::new(&handle, "main", tauri::WindowUrl::App("index.html".into()))
            .title("ImageShrink")
            .inner_size(800.0, 600.0)
            .fullscreen(false)
            .resizable(true);

    window_builder = window_builder.menu(main_menu.get_menu().clone());

    let win = window_builder.build()?;
    let win_clone = win.clone();
    win.on_menu_event(move |event| {
        let id = event.menu_item_id();
        match id {
            "reload" => {
                win_clone.eval("window.location.reload()").unwrap();
            }
            "about" => {
                let handle = win_clone.app_handle();

                let window = if handle.get_window("about").is_none() {
                    WindowBuilder::new(&handle, "about", tauri::WindowUrl::App("about".into()))
                        .title("About ImageShrink")
                        .inner_size(300.0, 300.0)
                        .fullscreen(false)
                        .resizable(true)
                        .build()
                        .unwrap()
                } else {
                    handle.get_window("about").unwrap()
                };

                window.show().unwrap();
                window.set_focus().unwrap();
            }
            "dev_tools" => {
                win_clone.open_devtools();
            }
            _ => {}
        }
    });
    Ok(())
}
