mod arrow;
mod options;
mod rect;
mod text;

use crate::types::Point;

pub use arrow::State as ArrowDetails;
pub use rect::State as RectDetails;
#[allow(unused_imports)]
pub use text::State as TextDetails;

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeDetails {
    Arrow(arrow::State),
    Rect(rect::State),
    #[allow(unused)]
    Text(text::State),
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Options {
    pub stroke_color: options::Color,
    pub roughness: options::Roughness,
    pub thickness: options::Thickness,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeConfig {
    pub start: Point,
    pub end: Point,
    pub details: ShapeDetails,
    pub options: Options,
}

#[derive(Clone, Debug)]
pub struct PartialShapeConfig {
    pub start: Option<Point>,
    pub end: Option<Point>,
    pub details: Option<ShapeDetails>,
    pub options: Option<Options>,
}

impl ShapeConfig {
    pub fn update(&mut self, update: PartialShapeConfig) {
        if let Some(top_left) = update.start {
            self.start = top_left;
        }

        if let Some(bottom_right) = update.end {
            self.end = bottom_right;
        }

        if let Some(details) = update.details {
            self.details = details;
        }

        if let Some(options) = update.options {
            self.options = options;
        }
    }
}
