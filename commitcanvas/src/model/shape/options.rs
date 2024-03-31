use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
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

impl From<&Color> for &'static str {
    fn from(color: &Color) -> &'static str {
        match color {
            Color::None => "cc_fill_none",
            Color::Red => "cc_fill_red",
            Color::Organge => "cc_fill_orange",
            Color::Amber => "cc_fill_amber",
            Color::Yellow => "cc_fill_yellow",
            Color::Lime => "cc_fill_lime",
            Color::Green => "cc_fill_green",
            Color::Emerald => "cc_fill_emerald",
            Color::Teal => "cc_fill_teal",
            Color::Cyan => "cc_fill_cyan",
            Color::Sky => "cc_fill_sky",
            Color::Blue => "cc_fill_blue",
            Color::Indigo => "cc_fill_indigo",
            Color::Purple => "cc_fill_purple",
            Color::Fuchsia => "cc_fill_fuchsia",
            Color::Pink => "cc_fill_pink",
            Color::Rose => "cc_fill_rose",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
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
