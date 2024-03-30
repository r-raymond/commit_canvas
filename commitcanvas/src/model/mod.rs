mod events;
mod guid;
mod shape;

use std::collections::HashMap;

pub use crate::types::Guid;
use crate::view::View;

pub use events::{Event, EventHistory};
pub use shape::ArrowDetails;
pub use shape::RectDetails;
pub use shape::Shape;
pub use shape::ShapeDetails;
pub use shape::TextDetails;

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
            Event::AddShape { data } => {
                let guid = if let Some(guid) = data.guid {
                    guid
                } else {
                    self.guid_generator.next()
                };
                log::info!("Adding shape: {guid}");
                let shape = Shape::new(
                    guid,
                    data.top_left,
                    data.bottom_right,
                    data.details,
                    data.options,
                );
                self.shapes.insert(guid, shape.clone());
                Some(EventHistory::AddShape { shape })
            }
            Event::RemoveShape { guid } => {
                log::info!("Removing shape: {guid}");
                self.shapes
                    .remove(&guid)
                    .map(|shape| EventHistory::RemoveShape { shape })
            }
            Event::ModifyShape { guid, data } => {
                log::info!("Modifying shape: {guid}");
                self.shapes.get_mut(&guid).map(|shape| {
                    let old_shape = shape.clone();
                    shape.update(data);
                    EventHistory::ModifyShape {
                        from: old_shape,
                        to: shape.clone(),
                    }
                })
            }
        };

        if let Some(event) = &history {
            for view in self.views.iter_mut() {
                view.process_event(crate::view::Event::Modify {
                    event: event.clone(),
                });
            }
        }

        history
    }

    pub fn undo(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            if let Some(history) = self.history.get(self.history_index) {
                log::info!("Undoing event");
                let event = Event::from(history.clone().revert());
                self.apply(event);
            }
        }
    }

    pub fn redo(&mut self) {
        if self.history_index < self.history.len() {
            if let Some(history) = self.history.get(self.history_index) {
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
        view.process_event(crate::view::Event::Reload {
            shapes: self.shapes.values().collect(),
        });
        self.views.push(view);
    }

    pub fn reload_views(&mut self) {
        for view in self.views.iter_mut() {
            view.process_event(crate::view::Event::Reload {
                shapes: self.shapes.values().collect(),
            });
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
            top_left: crate::types::Point { x: 0.0, y: 0.0 },
            bottom_right: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::AddShape { data: data.clone() };

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
            top_left: crate::types::Point { x: 0.0, y: 0.0 },
            bottom_right: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Rect(super::shape::RectDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::AddShape { data: data.clone() };

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
            top_left: crate::types::Point { x: 0.0, y: 0.0 },
            bottom_right: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Text(super::shape::TextDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::AddShape { data: data.clone() };

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
            top_left: crate::types::Point { x: 0.0, y: 0.0 },
            bottom_right: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event1 = Event::AddShape {
            data: data1.clone(),
        };

        let mut data2 = shape::ShapeCreate {
            guid: None,
            top_left: crate::types::Point { x: 0.0, y: 0.0 },
            bottom_right: crate::types::Point { x: 20.0, y: 20.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event2 = Event::AddShape {
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
    fn test_redo() {
        let mut model = Model::new();
        let mut data = shape::ShapeCreate {
            guid: None,
            top_left: crate::types::Point { x: 0.0, y: 0.0 },
            bottom_right: crate::types::Point { x: 10.0, y: 10.0 },
            details: super::shape::ShapeDetails::Arrow(super::shape::ArrowDetails::default()),
            options: super::shape::Options::default(),
        };
        let event = Event::AddShape { data: data.clone() };

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
