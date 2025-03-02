#[derive(Clone, Default, Debug, PartialEq)]
pub struct State {
    pub content: String,
    pub font_size: FontSize,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum FontSize {
    #[default]
    Medium,
    Small,
    Large,
}

impl From<&FontSize> for &'static str {
    fn from(size: &FontSize) -> &'static str {
        match size {
            FontSize::Small => "12px",
            FontSize::Medium => "16px",
            FontSize::Large => "24px",
        }
    }
}
