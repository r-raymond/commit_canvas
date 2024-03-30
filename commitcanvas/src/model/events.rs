use super::{
    shape::{Shape, ShapeCreate, ShapeUpdate},
    Guid,
};

#[derive(Clone, Debug)]
pub enum Event {
    AddShape { data: ShapeCreate },
    RemoveShape { guid: Guid },
    ModifyShape { guid: Guid, data: ShapeUpdate },
}

#[derive(Clone, Debug)]
pub enum EventHistory {
    AddShape { shape: Shape },
    RemoveShape { shape: Shape },
    ModifyShape { from: Shape, to: Shape },
}

impl EventHistory {
    pub fn guid(&self) -> Guid {
        match self {
            EventHistory::AddShape { shape } => shape.guid,
            EventHistory::RemoveShape { shape } => shape.guid,
            EventHistory::ModifyShape { from, .. } => from.guid,
        }
    }

    pub fn revert(&self) -> EventHistory {
        match self {
            EventHistory::AddShape { shape } => EventHistory::RemoveShape {
                shape: shape.clone(),
            },
            EventHistory::RemoveShape { shape } => EventHistory::AddShape {
                shape: shape.clone(),
            },
            EventHistory::ModifyShape { from, to } => EventHistory::ModifyShape {
                from: to.clone(),
                to: from.clone(),
            },
        }
    }
}

impl From<EventHistory> for Event {
    fn from(event: EventHistory) -> Self {
        match event {
            EventHistory::AddShape { shape } => Event::AddShape {
                data: ShapeCreate::from(shape),
            },
            EventHistory::RemoveShape { shape } => Event::RemoveShape { guid: shape.guid },
            EventHistory::ModifyShape { from, to } => Event::ModifyShape {
                guid: from.guid,
                data: ShapeUpdate::from(to),
            },
        }
    }
}
