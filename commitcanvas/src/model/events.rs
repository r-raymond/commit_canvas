use super::{
    shape::{Shape, ShapeCreate, ShapeUpdate},
    Guid,
};

#[derive(Clone, Debug)]
pub enum Event {
    Add { data: ShapeCreate },
    Remove { guid: Guid },
    Modify { guid: Guid, data: ShapeUpdate },
}

#[derive(Clone, Debug)]
pub enum EventHistory {
    Add { shape: Shape },
    Remove { shape: Shape },
    Modify { from: Shape, to: Shape },
}

impl EventHistory {
    pub fn guid(&self) -> Guid {
        match self {
            EventHistory::Add { shape } => shape.guid,
            EventHistory::Remove { shape } => shape.guid,
            EventHistory::Modify { from, .. } => from.guid,
        }
    }

    pub fn revert(&self) -> EventHistory {
        match self {
            EventHistory::Add { shape } => EventHistory::Remove {
                shape: shape.clone(),
            },
            EventHistory::Remove { shape } => EventHistory::Add {
                shape: shape.clone(),
            },
            EventHistory::Modify { from, to } => EventHistory::Modify {
                from: to.clone(),
                to: from.clone(),
            },
        }
    }
}

impl From<EventHistory> for Event {
    fn from(event: EventHistory) -> Self {
        match event {
            EventHistory::Add { shape } => Event::Add {
                data: ShapeCreate::from(shape),
            },
            EventHistory::Remove { shape } => Event::Remove { guid: shape.guid },
            EventHistory::Modify { from, to } => Event::Modify {
                guid: from.guid,
                data: ShapeUpdate::from(to),
            },
        }
    }
}
