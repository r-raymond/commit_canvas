mod events;
mod guid;
mod shape;

use std::collections::HashMap;

pub use crate::types::Guid;
use crate::view::View;

pub use events::{Event, EventHistory};

pub use shape::{ArrowDetails, Options, RectDetails, ShapeDetails};
pub use shape::{PartialShapeConfig, ShapeConfig};

pub struct Model {
    guid_generator: guid::GuidGenerator,
    shapes: HashMap<Guid, shape::ShapeConfig>,
    history: Vec<EventHistory>,
    history_index: usize,
    views: Vec<Box<dyn View>>,
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Model {
    pub fn new() -> Self {
        Self {
            guid_generator: guid::GuidGenerator::new(),
            shapes: HashMap::new(),
            history: Vec::new(),
            history_index: 0,
            views: Vec::new(),
        }
    }

    fn add_to_history(&mut self, history: EventHistory) {
        self.history.truncate(self.history_index);
        self.history_index += 1;
        self.history.push(history);
    }

    pub fn process_event(&mut self, event: Event) -> Option<Guid> {
        self.apply(event).and_then(|history| {
            self.add_to_history(history.clone());
            history.guid()
        })
    }

    fn apply(&mut self, event: Event) -> Option<EventHistory> {
        let history = match event {
            Event::Add { guid, config } => {
                let guid = if let Some(guid) = guid {
                    guid
                } else {
                    self.guid_generator.next()
                };
                log::info!("adding shape: {guid}");
                self.shapes.insert(guid, config.clone());
                Some(EventHistory::Add { guid, config })
            }
            Event::Remove { guid } => {
                log::info!("removing shape: {guid}");
                self.shapes
                    .remove(&guid)
                    .map(|config| EventHistory::Remove { guid, config })
            }
            Event::Modify { guid, config } => {
                log::debug!("modifying shape: {guid}");
                self.shapes.get_mut(&guid).and_then(|current_config| {
                    let old_config = current_config.clone();
                    current_config.update(config);

                    if let Some(EventHistory::Modify {
                        guid: shape_guid,
                        to,
                        ..
                    }) = self.history.last_mut()
                    {
                        if *shape_guid == guid {
                            *to = current_config.clone();
                            return None;
                        }
                    }
                    Some(EventHistory::Modify {
                        guid,
                        from: old_config,
                        to: current_config.clone(),
                    })
                })
            }
            Event::Checkpoint => None,
        };

        if let Some(event) = &history {
            for view in self.views.iter_mut() {
                if let Err(e) = view.process_event(crate::view::Event::Modify {
                    event: event.clone(),
                }) {
                    log::warn!("Error updating view {:?}", e);
                }
            }
        } else if let Some(event) = self.history.last() {
            for view in self.views.iter_mut() {
                if let Err(e) = view.process_event(crate::view::Event::Modify {
                    event: event.clone(),
                }) {
                    log::warn!("Error updating view {:?}", e);
                }
            }
        }

        history
    }

    pub fn undo(&mut self) {
        log::info!("calling model undo");
        if self.history_index > 0 {
            self.history_index -= 1;
            if let Some(history) = self.history.get(self.history_index) {
                log::info!("undoing event");
                let event = Event::from(history.clone().revert());
                self.apply(event);
            }
        }
    }

    pub fn redo(&mut self) {
        log::info!("calling model redo");
        if self.history_index < self.history.len() {
            if let Some(history) = self.history.get(self.history_index) {
                log::info!("redoing event");
                self.history_index += 1;
                let event = Event::from(history.clone());
                self.apply(event);
            }
        }
    }

    pub fn get_shape(&self, guid: Guid) -> Option<&shape::ShapeConfig> {
        self.shapes.get(&guid)
    }

    pub fn add_view(&mut self, mut view: Box<dyn View>) {
        if let Err(e) = view.process_event(crate::view::Event::Reload {
            shapes: self.shapes.iter().collect(),
        }) {
            log::warn!("Error updating view {:?}", e);
        }
        self.views.push(view);
    }

    #[allow(dead_code)]
    pub fn reload_views(&mut self) {
        for view in self.views.iter_mut() {
            if let Err(e) = view.process_event(crate::view::Event::Reload {
                shapes: self.shapes.iter().collect(),
            }) {
                log::warn!("Error updating view {:?}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_arrow() {
        let mut model = Model::new();
        let config = shape::ShapeConfig {
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::Add {
            guid: None,
            config: config.clone(),
        };

        let guid = model.process_event(event);

        assert!(guid.is_some());
        assert!(model.get_shape(guid.unwrap()).is_some());
        assert_eq!(*model.get_shape(guid.unwrap()).unwrap(), config);
    }

    #[test]
    fn test_create_rect() {
        let mut model = Model::new();
        let config = shape::ShapeConfig {
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Rect(super::shape::RectDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::Add {
            guid: None,
            config: config.clone(),
        };

        let guid = model.process_event(event);

        assert!(guid.is_some());
        assert!(model.get_shape(guid.unwrap()).is_some());
        assert_eq!(*model.get_shape(guid.unwrap()).unwrap(), config);
    }

    #[test]
    fn test_create_text() {
        let mut model = Model::new();
        let config = shape::ShapeConfig {
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Text(super::shape::TextDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::Add {
            config: config.clone(),
            guid: None,
        };

        let guid = model.process_event(event);

        assert!(guid.is_some());
        assert!(model.get_shape(guid.unwrap()).is_some());
        assert_eq!(*model.get_shape(guid.unwrap()).unwrap(), config);
    }

    #[test]
    fn test_undo() {
        let mut model = Model::new();
        let config1 = shape::ShapeConfig {
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event1 = Event::Add {
            guid: None,
            config: config1.clone(),
        };

        let config2 = shape::ShapeConfig {
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 20.0, y: 20.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event2 = Event::Add {
            guid: None,
            config: config2.clone(),
        };

        let guid1 = model.process_event(event1);
        let guid2 = model.process_event(event2);

        assert!(guid1.is_some());
        assert!(model.get_shape(guid1.unwrap()).is_some());
        assert_eq!(*model.get_shape(guid1.unwrap()).unwrap(), config1);

        assert!(guid2.is_some());
        assert!(model.get_shape(guid2.unwrap()).is_some());
        assert_eq!(*model.get_shape(guid2.unwrap()).unwrap(), config2);

        model.undo();

        assert!(model.get_shape(guid2.unwrap()).is_none());

        assert!(guid1.is_some());
        assert!(model.get_shape(guid1.unwrap()).is_some());
        assert_eq!(*model.get_shape(guid1.unwrap()).unwrap(), config1);
    }

    #[test]
    fn test_undo_commit() {
        let mut model = Model::new();
        let config1 = shape::ShapeConfig {
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event1 = Event::Add {
            guid: None,
            config: config1.clone(),
        };

        let guid1 = model.process_event(event1);
        model.process_event(Event::Checkpoint);

        assert!(guid1.is_some());

        let mod1 = shape::PartialShapeConfig {
            start: None,
            end: Some(crate::types::Point { x: 20.0, y: 20.0 }),
            details: None,
            options: None,
        };

        let mod2 = shape::PartialShapeConfig {
            start: None,
            end: Some(crate::types::Point { x: 30.0, y: 30.0 }),
            details: None,
            options: None,
        };

        assert!(model.get_shape(guid1.unwrap()).is_some());
        assert!(
            model.get_shape(guid1.unwrap()).unwrap().end
                == crate::types::Point { x: 10.0, y: 10.0 }
        );

        model.process_event(Event::Modify {
            guid: guid1.unwrap(),
            config: mod1,
        });

        model.process_event(Event::Modify {
            guid: guid1.unwrap(),
            config: mod2,
        });
        model.process_event(Event::Checkpoint);

        assert!(model.get_shape(guid1.unwrap()).is_some());
        assert!(
            model.get_shape(guid1.unwrap()).unwrap().end
                == crate::types::Point { x: 30.0, y: 30.0 }
        );

        model.undo();

        assert!(model.get_shape(guid1.unwrap()).is_some());
        assert!(
            model.get_shape(guid1.unwrap()).unwrap().end
                == crate::types::Point { x: 10.0, y: 10.0 }
        );
    }

    #[test]
    fn test_create_modify_undo() {
        let mut model = Model::new();
        let config1 = shape::ShapeConfig {
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event1 = Event::Add {
            guid: None,
            config: config1,
        };

        let guid1 = model.process_event(event1);

        let mod1 = shape::PartialShapeConfig {
            start: None,
            end: Some(crate::types::Point { x: 20.0, y: 20.0 }),
            details: None,
            options: None,
        };

        model.process_event(Event::Modify {
            guid: guid1.unwrap(),
            config: mod1,
        });

        model.process_event(Event::Checkpoint);

        assert!(model.get_shape(guid1.unwrap()).is_some());
        assert!(
            model.get_shape(guid1.unwrap()).unwrap().end
                == crate::types::Point { x: 20.0, y: 20.0 }
        );

        model.undo();

        //assert!(model.get_shape(guid1.unwrap()).is_none());
    }

    #[test]
    fn test_redo() {
        let mut model = Model::new();
        let config = shape::ShapeConfig {
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::Add {
            guid: None,
            config: config.clone(),
        };

        let guid = model.process_event(event);

        assert!(guid.is_some());
        assert!(model.get_shape(guid.unwrap()).is_some());
        assert_eq!(*model.get_shape(guid.unwrap()).unwrap(), config);

        model.undo();

        assert!(model.get_shape(guid.unwrap()).is_none());

        model.redo();

        assert!(model.get_shape(guid.unwrap()).is_some());
        assert_eq!(*model.get_shape(guid.unwrap()).unwrap(), config);
    }
}
