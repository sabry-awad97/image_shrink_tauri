use std::error::Error;
use tauri::{App, CustomMenuItem, Menu, MenuItem, Submenu, WindowBuilder};

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    let file_submenu = Submenu::new(
        "File",
        Menu::with_items([
            CustomMenuItem::new("reload", "Reload")
                .accelerator("CmdOrCtrl+R")
                .into(),
            MenuItem::Separator.into(),
            MenuItem::Quit.into(),
        ]),
    );
    let menu = Menu::new().add_submenu(file_submenu);

    let handle = app.handle();
    let mut window_builder =
        WindowBuilder::new(&handle, "main", tauri::WindowUrl::App("index.html".into()))
            .title("ImageShrink")
            .inner_size(800.0, 600.0)
            .fullscreen(false)
            .resizable(true);

    window_builder = window_builder.menu(menu);

    let win = window_builder.build()?;
    let win_clone = win.clone();
    win.on_menu_event(move |event| {
        let id = event.menu_item_id();
        match id {
            "reload" => {
                win_clone.eval("window.location.reload()").unwrap();
            }
            "about" => {}
            _ => {}
        }
    });
    Ok(())
}
