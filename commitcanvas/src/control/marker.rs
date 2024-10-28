use std::error::Error;

use crate::types::PointPixel;

pub trait Marker {
    fn new() -> Result<Self, Box<dyn Error + Send + Sync>>
    where
        Self: Sized;
    fn update(&self, p: PointPixel) -> Result<(), Box<dyn Error + Send + Sync>>;
}
