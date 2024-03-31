use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[derive(Default)]
pub enum Color {
    #[default]
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



#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[derive(Default)]
pub enum Roughness {
    Low,
    #[default]
    Medium,
    High,
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
#[derive(Default)]
pub enum Thickness {
    #[default]
    Thin,
    Medium,
    Thick,
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
