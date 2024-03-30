mod arrow;
mod options;
mod rect;
mod text;

use crate::types::{Guid, Point};

pub use arrow::State as ArrowDetails;
pub use rect::State as RectDetails;
pub use text::State as TextDetails;

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeDetails {
    Arrow(arrow::State),
    Rect(rect::State),
    Text(text::State),
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Options {
    stroke_color: options::Color,
    roughness: options::Roughness,
    thickness: options::Thickness,
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub guid: Guid,
    pub top_left: Point,
    pub bottom_right: Point,
    pub details: ShapeDetails,
    pub options: Options,
}

#[derive(Clone, Debug)]
pub struct ShapeUpdate {
    pub guid: Guid,
    pub top_left: Option<Point>,
    pub bottom_right: Option<Point>,
    pub details: Option<ShapeDetails>,
    pub options: Option<Options>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeCreate {
    pub guid: Option<Guid>,
    pub top_left: Point,
    pub bottom_right: Point,
    pub details: ShapeDetails,
    pub options: Options,
}

impl Shape {
    pub fn new(
        guid: Guid,
        top_left: Point,
        bottom_right: Point,
        details: ShapeDetails,
        options: Options,
    ) -> Self {
        Self {
            guid,
            top_left,
            bottom_right,
            details,
            options,
        }
    }

    pub fn update(&mut self, update: ShapeUpdate) {
        if let Some(top_left) = update.top_left {
            self.top_left = top_left;
        }

        if let Some(bottom_right) = update.bottom_right {
            self.bottom_right = bottom_right;
        }

        if let Some(details) = update.details {
            self.details = details;
        }

        if let Some(options) = update.options {
            self.options = options;
        }
    }
}

impl From<Shape> for ShapeCreate {
    fn from(shape: Shape) -> Self {
        Self {
            guid: Some(shape.guid),
            top_left: shape.top_left,
            bottom_right: shape.bottom_right,
            details: shape.details,
            options: shape.options,
        }
    }
}

impl From<Shape> for ShapeUpdate {
    fn from(shape: Shape) -> Self {
        Self {
            guid: shape.guid,
            top_left: Some(shape.top_left),
            bottom_right: Some(shape.bottom_right),
            details: Some(shape.details),
            options: Some(shape.options),
        }
    }
}
