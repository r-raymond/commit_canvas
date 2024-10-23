mod events;
mod guid;
mod shape;

use std::collections::HashMap;

pub use crate::types::Guid;
use crate::view::View;

pub use events::{Event, EventHistory};

pub use shape::{ArrowDetails, Options, RectDetails, ShapeDetails};
pub use shape::{Shape, ShapeCreate, ShapeUpdate};

pub struct Model {
    guid_generator: guid::GuidGenerator,
    shapes: HashMap<Guid, shape::Shape>,
    history: Vec<EventHistory>,
    history_index: usize,
    views: Vec<Box<dyn View>>,
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
        self.apply(event).map(|history| {
            self.add_to_history(history.clone());
            history.guid()
        })
    }

    fn apply(&mut self, event: Event) -> Option<EventHistory> {
        let history = match event {
            Event::Add { data } => {
                let guid = if let Some(guid) = data.guid {
                    guid
                } else {
                    self.guid_generator.next()
                };
                log::info!("adding shape: {guid}");
                let shape = Shape::new(guid, data.start, data.end, data.details, data.options);
                self.shapes.insert(guid, shape.clone());
                Some(EventHistory::Add { shape })
            }
            Event::Remove { guid } => {
                log::info!("removing shape: {guid}");
                self.shapes
                    .remove(&guid)
                    .map(|shape| EventHistory::Remove { shape })
            }
            Event::Modify { guid, data } => {
                log::debug!("modifying shape: {guid}");
                self.shapes
                    .get_mut(&guid)
                    .map(|shape| {
                        let old_shape = shape.clone();
                        shape.update(data);

                        if let Some(history) = self.history.last_mut() {
                            if let EventHistory::Modify { from, to, commit } = history {
                                if *commit == false && from.guid == guid {
                                    *to = shape.clone();
                                    return None;
                                } else {
                                    *commit = true;
                                }
                            }
                        }
                        Some(EventHistory::Modify {
                            from: old_shape,
                            to: shape.clone(),
                            commit: false,
                        })
                    })
                    .flatten()
            }
            Event::Checkpoint => {
                if let Some(history) = self.history.last_mut() {
                    if let EventHistory::Modify { commit, .. } = history {
                        *commit = true;
                    }
                }
                None
            }
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

    pub fn get_shape(&self, guid: Guid) -> Option<&shape::Shape> {
        self.shapes.get(&guid)
    }

    pub fn add_view(&mut self, mut view: Box<dyn View>) {
        if let Err(e) = view.process_event(crate::view::Event::Reload {
            shapes: self.shapes.values().collect(),
        }) {
            log::warn!("Error updating view {:?}", e);
        }
        self.views.push(view);
    }

    #[allow(dead_code)]
    pub fn reload_views(&mut self) {
        for view in self.views.iter_mut() {
            if let Err(e) = view.process_event(crate::view::Event::Reload {
                shapes: self.shapes.values().collect(),
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
        let mut data = shape::ShapeCreate {
            guid: None,
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::Add { data: data.clone() };

        let guid = model.process_event(event);
        data.guid = guid;

        assert!(guid.is_some());
        assert!(model.get_shape(guid.unwrap()).is_some());
        assert_eq!(
            shape::ShapeCreate::from(model.get_shape(guid.unwrap()).unwrap().clone()),
            data
        );
    }

    #[test]
    fn test_create_rect() {
        let mut model = Model::new();
        let mut data = shape::ShapeCreate {
            guid: None,
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Rect(super::shape::RectDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::Add { data: data.clone() };

        let guid = model.process_event(event);
        data.guid = guid;

        assert!(guid.is_some());
        assert!(model.get_shape(guid.unwrap()).is_some());
        assert_eq!(
            shape::ShapeCreate::from(model.get_shape(guid.unwrap()).unwrap().clone()),
            data
        );
    }

    #[test]
    fn test_create_text() {
        let mut model = Model::new();
        let mut data = shape::ShapeCreate {
            guid: None,
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Text(super::shape::TextDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::Add { data: data.clone() };

        let guid = model.process_event(event);
        data.guid = guid;

        assert!(guid.is_some());
        assert!(model.get_shape(guid.unwrap()).is_some());
        assert_eq!(
            shape::ShapeCreate::from(model.get_shape(guid.unwrap()).unwrap().clone()),
            data
        );
    }

    #[test]
    fn test_undo() {
        let mut model = Model::new();
        let mut data1 = shape::ShapeCreate {
            guid: None,
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event1 = Event::Add {
            data: data1.clone(),
        };

        let mut data2 = shape::ShapeCreate {
            guid: None,
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 20.0, y: 20.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event2 = Event::Add {
            data: data2.clone(),
        };

        let guid1 = model.process_event(event1);
        let guid2 = model.process_event(event2);
        data1.guid = guid1;
        data2.guid = guid2;

        assert!(guid1.is_some());
        assert!(model.get_shape(guid1.unwrap()).is_some());
        assert_eq!(
            shape::ShapeCreate::from(model.get_shape(guid1.unwrap()).unwrap().clone()),
            data1
        );

        assert!(guid2.is_some());
        assert!(model.get_shape(guid2.unwrap()).is_some());
        assert_eq!(
            shape::ShapeCreate::from(model.get_shape(guid2.unwrap()).unwrap().clone()),
            data2
        );

        model.undo();

        assert!(model.get_shape(guid2.unwrap()).is_none());

        assert!(guid1.is_some());
        assert!(model.get_shape(guid1.unwrap()).is_some());
        assert_eq!(
            shape::ShapeCreate::from(model.get_shape(guid1.unwrap()).unwrap().clone()),
            data1
        );
    }

    #[test]
    fn test_undo_commit() {
        let mut model = Model::new();
        let data1 = shape::ShapeCreate {
            guid: None,
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event1 = Event::Add {
            data: data1.clone(),
        };

        let guid1 = model.process_event(event1);
        model.process_event(Event::Checkpoint);

        assert!(guid1.is_some());

        let mod1 = shape::ShapeUpdate {
            guid: guid1.unwrap(),
            start: None,
            end: Some(crate::types::Point { x: 20.0, y: 20.0 }),
            details: None,
            options: None,
        };

        let mod2 = shape::ShapeUpdate {
            guid: guid1.unwrap(),
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
            data: mod1,
        });

        model.process_event(Event::Modify {
            guid: guid1.unwrap(),
            data: mod2,
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
    fn test_redo() {
        let mut model = Model::new();
        let mut data = shape::ShapeCreate {
            guid: None,
            start: crate::types::Point { x: 0.0, y: 0.0 },
            end: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::Add { data: data.clone() };

        let guid = model.process_event(event);
        data.guid = guid;

        assert!(guid.is_some());
        assert!(model.get_shape(guid.unwrap()).is_some());
        assert_eq!(
            shape::ShapeCreate::from(model.get_shape(guid.unwrap()).unwrap().clone()),
            data
        );

        model.undo();

        assert!(model.get_shape(guid.unwrap()).is_none());

        model.redo();

        assert!(model.get_shape(guid.unwrap()).is_some());
        assert_eq!(
            shape::ShapeCreate::from(model.get_shape(guid.unwrap()).unwrap().clone()),
            data
        );
    }
}
