#[derive(PartialEq, Eq, Hash)]
pub enum MenuItemId {
    Reload,
    About,
    DevTools,
}

impl From<MenuItemId> for String {
    fn from(id: MenuItemId) -> Self {
        match id {
            MenuItemId::Reload => "Reload".to_string(),
            MenuItemId::About => "About".to_string(),
            MenuItemId::DevTools => "DevTools".to_string(),
        }
    }
}

impl From<&str> for MenuItemId {
    fn from(id: &str) -> Self {
        match id {
            "Reload" => MenuItemId::Reload,
            "About" => MenuItemId::About,
            "DevTools" => MenuItemId::DevTools,
            _ => unreachable!(),
        }
    }
}
