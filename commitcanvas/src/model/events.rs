use super::{
    shape::{PartialShapeConfig, ShapeConfig},
    Guid,
};

#[derive(Clone, Debug)]
pub enum Event {
    Add {
        /// For internal uses only. Don't set this field from the controller.
        guid: Option<Guid>,
        config: ShapeConfig,
    },
    Remove {
        guid: Guid,
    },
    Modify {
        guid: Guid,
        config: PartialShapeConfig,
    },
    Checkpoint,
}

#[derive(Clone, Debug)]
pub enum EventHistory {
    Add {
        guid: Guid,
        config: ShapeConfig,
    },
    Remove {
        guid: Guid,
        config: ShapeConfig,
    },
    Modify {
        guid: Guid,
        from: ShapeConfig,
        to: ShapeConfig,
    },
    #[allow(unused)]
    Checkpoint,
}

impl EventHistory {
    pub fn guid(&self) -> Option<Guid> {
        match self {
            EventHistory::Add { guid, .. } => Some(*guid),
            EventHistory::Remove { guid, .. } => Some(*guid),
            EventHistory::Modify { guid, .. } => Some(*guid),
            EventHistory::Checkpoint => None,
        }
    }

    pub fn revert(&self) -> EventHistory {
        match self {
            EventHistory::Add { guid, config } => EventHistory::Remove {
                guid: *guid,
                config: config.clone(),
            },
            EventHistory::Remove { guid, config } => EventHistory::Add {
                guid: *guid,
                config: config.clone(),
            },
            EventHistory::Modify { guid, from, to } => EventHistory::Modify {
                guid: *guid,
                from: to.clone(),
                to: from.clone(),
            },
            EventHistory::Checkpoint => EventHistory::Checkpoint,
        }
    }

    #[allow(unused)]
    pub fn fold(&self, other: &EventHistory) -> Option<EventHistory> {
        None
    }
}

impl From<EventHistory> for Event {
    fn from(event: EventHistory) -> Self {
        match event {
            EventHistory::Add { guid, config } => Event::Add {
                guid: Some(guid),
                config,
            },
            EventHistory::Remove { guid, .. } => Event::Remove { guid },
            EventHistory::Modify { guid, to, .. } => Event::Modify {
                guid,
                config: PartialShapeConfig {
                    start: Some(to.start),
                    end: Some(to.end),
                    details: Some(to.details),
                    options: Some(to.options),
                },
            },
            EventHistory::Checkpoint => Event::Checkpoint,
        }
    }
}
