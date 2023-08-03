use std::fmt;

#[derive(PartialEq, Eq, Hash)]
pub enum MenuItemId {
    Reload,
    About,
    DevTools,
}

impl fmt::Display for MenuItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuItemId::Reload => write!(f, "Reload"),
            MenuItemId::About => write!(f, "About"),
            MenuItemId::DevTools => write!(f, "DevTools"),
        }
    }
}

impl From<MenuItemId> for String {
    fn from(id: MenuItemId) -> Self {
        id.to_string()
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
