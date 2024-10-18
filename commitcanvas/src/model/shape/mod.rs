mod arrow;
mod options;
mod rect;
mod text;

use crate::types::{Guid, Point};

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

#[derive(Clone, Debug)]
pub struct Shape {
    pub guid: Guid,
    pub start: Point, // TODO: the model should store coords, not pixels!
    pub end: Point,
    pub details: ShapeDetails,
    pub options: Options,
}

#[derive(Clone, Debug)]
pub struct ShapeUpdate {
    #[allow(dead_code)]
    pub guid: Guid,
    pub start: Option<Point>,
    pub end: Option<Point>,
    pub details: Option<ShapeDetails>,
    pub options: Option<Options>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeCreate {
    pub guid: Option<Guid>,
    pub start: Point,
    pub end: Point,
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
            start: top_left,
            end: bottom_right,
            details,
            options,
        }
    }

    pub fn update(&mut self, update: ShapeUpdate) {
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

impl From<Shape> for ShapeCreate {
    fn from(shape: Shape) -> Self {
        Self {
            guid: Some(shape.guid),
            start: shape.start,
            end: shape.end,
            details: shape.details,
            options: shape.options,
        }
    }
}

impl From<Shape> for ShapeUpdate {
    fn from(shape: Shape) -> Self {
        Self {
            guid: shape.guid,
            start: Some(shape.start),
            end: Some(shape.end),
            details: Some(shape.details),
            options: Some(shape.options),
        }
    }
}
