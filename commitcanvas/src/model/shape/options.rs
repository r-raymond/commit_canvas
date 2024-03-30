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

impl From<&Roughness> for f32 {
    fn from(roughness: &Roughness) -> f32 {
        match roughness {
            Roughness::Low => 0.0,
            Roughness::Medium => 0.4,
            Roughness::High => 0.8,
        }
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

impl From<&Thickness> for &'static str {
    fn from(thickness: &Thickness) -> &'static str {
        match thickness {
            Thickness::Thin => "1.0",
            Thickness::Medium => "2.0",
            Thickness::Thick => "3.0",
        }
    }
}
