#[derive(PartialEq, Eq, Hash)]
pub enum MenuId {
    About,
    Reload,
    DevTools,
}

impl MenuId {
    pub fn as_str(&self) -> &str {
        match self {
            Self::About => "about",
            Self::Reload => "reload",
            Self::DevTools => "dev_tools",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "about" => Some(Self::About),
            "reload" => Some(Self::Reload),
            "dev_tools" => Some(Self::DevTools),
            _ => None,
        }
    }
}
