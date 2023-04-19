use tauri::CustomMenuItem;

use super::menu_id::MenuId;

pub struct MenuItemInfo {
    pub id: MenuId,
    title: String,
    accelerator: Option<String>,
    pub handler: Box<dyn Fn(&tauri::Window) + Send + Sync>,
}

impl MenuItemInfo {
    pub fn new(
        id: MenuId,
        title: &str,
        handler: impl Fn(&tauri::Window) + Send + Sync + 'static,
    ) -> Self {
        Self {
            id,
            title: title.to_string(),
            accelerator: None,
            handler: Box::new(handler),
        }
    }

    pub fn with_accelerator(mut self, accelerator: &str) -> Self {
        self.accelerator = Some(accelerator.to_string());
        self
    }
}

impl Into<tauri::MenuEntry> for MenuItemInfo {
    fn into(self) -> tauri::MenuEntry {
        let mut item = CustomMenuItem::new(self.id.as_str(), &self.title);
        if let Some(accelerator) = self.accelerator {
            item = item.accelerator(&accelerator);
        }
        item.into()
    }
}
