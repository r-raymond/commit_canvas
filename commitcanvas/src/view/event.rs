use crate::{
    model::{EventHistory, Guid, ShapeConfig},
    types::VecPixel,
};

pub enum Event<'a> {
    Reload {
        shapes: Vec<(&'a Guid, &'a ShapeConfig)>,
    },
    Modify {
        event: EventHistory,
    },
    Pan {
        vec: VecPixel,
    },
}
