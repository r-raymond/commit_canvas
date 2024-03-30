use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Color {
    None,
    Red,
    Organge,
    Amber,
    Yellow,
    Lime,
    Green,
    Emerald,
    Teal,
    Cyan,
    Sky,
    Blue,
    Indigo,
    Purple,
    Fuchsia,
    Pink,
    Rose,
}

impl Default for Color {
    fn default() -> Self {
        Color::None
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Roughness {
    Low,
    Medium,
    High,
}

impl Default for Roughness {
    fn default() -> Self {
        Roughness::Medium
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Thickness {
    Thin,
    Medium,
    Thick,
}

impl Default for Thickness {
    fn default() -> Self {
        Thickness::Thin
    }
}
