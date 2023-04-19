use std::collections::HashMap;

use tauri::{Manager, Menu, MenuItem, Submenu};

use crate::app::{
    constants::{
        ABOUT_TITLE, DEVTOOLS_ACCELERATOR, DEVTOOLS_TITLE, RELOAD_ACCELERATOR, RELOAD_TITLE,
    },
    window,
};

use self::menu_item_info::MenuItemInfo;

use super::{
    constants::{FILE_MENU_TITLE, HELP_MENU_TITLE},
    state::{DevToolsState, APP_STATE},
};

use menu_id::MenuId;
mod menu_id;
mod menu_item_info;

fn file_menu_items() -> Vec<MenuItemInfo> {
    vec![MenuItemInfo::new(MenuId::Reload, RELOAD_TITLE, |win| {
        win.eval("window.location.reload()").unwrap();
    })
    .with_accelerator(RELOAD_ACCELERATOR)]
}

fn help_menu_items() -> Vec<MenuItemInfo> {
    vec![
        MenuItemInfo::new(MenuId::About, ABOUT_TITLE, |win| {
            let win = win.clone();
            let handle = win.app_handle();
            window::create_about_window(&handle)
        }),
        MenuItemInfo::new(MenuId::DevTools, DEVTOOLS_TITLE, |win| {
            let mut dev_tools_state = APP_STATE.dev_tools_state.lock().unwrap();
            match *dev_tools_state {
                DevToolsState::Closed => {
                    win.open_devtools();
                    *dev_tools_state = DevToolsState::Opened;
                }
                DevToolsState::Opened => {
                    win.close_devtools();
                    *dev_tools_state = DevToolsState::Closed;
                }
            }
        })
        .with_accelerator(DEVTOOLS_ACCELERATOR),
    ]
}

pub fn init() -> Menu {
    let file_menu = Submenu::new(
        FILE_MENU_TITLE,
        Menu::with_items(
            file_menu_items()
                .into_iter()
                .map(|info| info.into())
                .chain(vec![MenuItem::Separator.into(), MenuItem::Quit.into()]),
        ),
    );
    let help_menu = Submenu::new(
        HELP_MENU_TITLE,
        Menu::with_items(help_menu_items().into_iter().map(|info| info.into())),
    );
    Menu::new().add_submenu(file_menu).add_submenu(help_menu)
}

pub fn menu_handler(win: tauri::Window, event: tauri::MenuEvent) {
    let menu_items = file_menu_items()
        .into_iter()
        .chain(help_menu_items().into_iter())
        .map(|item| (item.id, item.handler))
        .collect::<HashMap<_, _>>();

    let id = event.menu_item_id();
    match MenuId::from_id(id) {
        Some(menu_id) => {
            if let Some(handler) = menu_items.get(&menu_id) {
                handler(&win)
            }
        }
        None => {
            println!("Got menu event: {}", id);
        }
    }
}
