pub mod menu_item_id;

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

use self::menu_item_id::MenuItemId;

pub struct MainMenu {
    menu: Menu,
}

impl MainMenu {
    pub fn new() -> Self {
        let menu = Menu::new()
            .add_submenu(Self::file_submenu())
            .add_submenu(Self::help_submenu());

        MainMenu { menu }
    }

    pub fn get_menu(&self) -> &Menu {
        &self.menu
    }

    fn file_submenu() -> Submenu {
        Submenu::new(
            "File",
            Menu::with_items([
                CustomMenuItem::new(MenuItemId::Reload, "Reload")
                    .accelerator("CmdOrCtrl+R")
                    .into(),
                MenuItem::Separator.into(),
                MenuItem::Quit.into(),
            ]),
        )
    }

    fn help_submenu() -> Submenu {
        Submenu::new(
            "Help",
            Menu::with_items([
                CustomMenuItem::new(MenuItemId::About, "About").into(),
                CustomMenuItem::new(MenuItemId::DevTools, "Open Developer Tools")
                    .accelerator("CmdOrCtrl+Shift+I")
                    .into(),
            ]),
        )
    }
}
