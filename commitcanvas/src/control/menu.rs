use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MainMenuButton {
    Arrow,
    Rect,
    Text,
    #[default]
    Select,
}

pub type MainMenuUpdate = Box<dyn Fn(MainMenuButton) -> Result<(), Box<dyn Error + Send + Sync>>>;
