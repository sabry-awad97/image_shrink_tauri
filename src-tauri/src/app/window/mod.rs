use tauri::{AppHandle, Manager, MenuEvent, WindowBuilder};

use super::{
    constants::{
        ABOUT_WINDOW_LABEL, ABOUT_WINDOW_TITLE, ABOUT_WINDOW_URL, MAIN_WINDOW_LABEL,
        MAIN_WINDOW_TITLE, MAIN_WINDOW_URL,
    },
    menu::{self, menu_handler},
};

pub struct AppWindow {
    pub window: tauri::Window,
}

impl AppWindow {
    pub fn new(
        handle: &AppHandle,
        label: &str,
        title: &str,
        url: &str,
        width: f64,
        height: f64,
        fullscreen: bool,
        resizable: bool,
        menu: Option<tauri::Menu>,
    ) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        if handle.get_window(label).is_none() {
            let mut window_builder =
                WindowBuilder::new(handle, label, tauri::WindowUrl::App(url.into()))
                    .title(title)
                    .inner_size(width, height)
                    .fullscreen(fullscreen)
                    .resizable(resizable);

            if let Some(menu) = menu {
                window_builder = window_builder.menu(menu);
            }

            let window = window_builder.build()?;
            return Ok(Self { window });
        }

        let window = handle.get_window(label).unwrap();
        window.show().unwrap();
        window.set_focus().unwrap();
        Ok(Self { window })
    }

    pub fn on_menu_event<F>(&self, handler: F)
    where
        F: Fn(&MenuEvent) + Send + 'static,
    {
        self.window.on_menu_event(move |event| {
            handler(&event);
        });
    }
}

pub fn create_about_window(handle: &AppHandle) {
    let handle = handle.clone();
    tauri::async_runtime::spawn(async move {
        AppWindow::new(
            &handle,
            ABOUT_WINDOW_LABEL,
            ABOUT_WINDOW_TITLE,
            ABOUT_WINDOW_URL,
            300.0,
            300.0,
            false,
            false,
            None,
        )
        .unwrap();
    });
}

pub fn create_main_window(handle: &AppHandle) {
    let handle = handle.clone();
    tauri::async_runtime::spawn(async move {
        let main_window = AppWindow::new(
            &handle,
            MAIN_WINDOW_LABEL,
            MAIN_WINDOW_TITLE,
            MAIN_WINDOW_URL,
            500.0,
            600.0,
            false,
            true,
            Some(menu::init()),
        )
        .unwrap();

        let win = main_window.window.clone();
        main_window.on_menu_event(move |event| {
            menu_handler(win.clone(), event.clone());
        });
    });
}
