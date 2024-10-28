use std::error::Error;

use crate::model::ShapeConfig;
use crate::types::Guid;

pub trait Selection {
    fn new(guid: Guid, config: &ShapeConfig) -> Result<Self, Box<dyn Error + Send + Sync>>
    where
        Self: Sized;
    fn update(&mut self, config: &ShapeConfig) -> Result<(), Box<dyn Error + Send + Sync>>;
}
