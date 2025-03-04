use crate::model::{EventHistory, Guid, ShapeConfig};

pub enum Event<'a> {
    Reload {
        shapes: Vec<(&'a Guid, &'a ShapeConfig)>,
    },
    Modify {
        event: EventHistory,
    },
}
