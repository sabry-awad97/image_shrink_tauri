use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

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
                CustomMenuItem::new("reload", "Reload")
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
                CustomMenuItem::new("about", "About").into(),
                CustomMenuItem::new("dev_tools", "Open Developer Tools")
                    .accelerator("CmdOrCtrl+Shift+I")
                    .into(),
            ]),
        )
    }
}
