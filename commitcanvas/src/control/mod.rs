use crate::{
    control::selection::Selection,
    model::{
        ArrowDetails, Event, Guid, Model, Options, RectDetails, Shape, ShapeCreate, ShapeDetails,
        ShapeUpdate,
    },
    utils::{coords_to_pixels, pixels_to_coords},
    view::UIView,
};

use self::menu::{setup, update_main_menu, MainMenuButton};

mod callback;
mod marker;
mod menu;
mod selection;

#[derive(Debug, Clone, Copy)]
pub enum ModificationType {
    TL,
    TR,
    BL,
    BR,
    T,
    R,
    B,
    L,
}

#[derive(Debug, Default)]
enum State {
    #[default]
    Normal,
    Selected {
        guid: Guid,
    },
    Modifying {
        guid: Guid,
        modification_type: ModificationType,
    },
}

pub struct Control {
    button_state: MainMenuButton,
    mouse_pixel_coords: (f32, f32),
    mouse_coords: (i32, i32),
    marker: Option<marker::Marker>,
    #[allow(dead_code)]
    selection: Option<selection::Selection>,
    model: Model,
    state: State,
    copied_shape: Option<Shape>,
}

impl Control {
    pub fn new() -> Control {
        log::info!("starting contol setup");
        setup().expect("failed to setup menu");
        callback::setup().expect("failed to setup callbacks");
        let button_state = MainMenuButton::default();
        update_main_menu(button_state).expect("failed to update main menu");

        let mut model = Model::new();
        model.add_view(Box::new(UIView::new()));

        Control {
            button_state: MainMenuButton::default(),
            mouse_pixel_coords: (0., 0.),
            mouse_coords: (0, 0),
            marker: None,
            selection: None,
            model,
            state: State::default(),
            copied_shape: None,
        }
    }

    pub fn set_button_state(&mut self, state: MainMenuButton) {
        log::info!("setting button state to {:?}", state);
        self.button_state = state;
        self.selection = None;
        update_main_menu(state).expect("failed to update main menu");
        match state {
            MainMenuButton::Arrow => {
                self.marker = Some(marker::Marker::new().expect("failed to create marker"));
            }
            MainMenuButton::Rect => {
                self.marker = Some(marker::Marker::new().expect("failed to create marker"));
            }
            MainMenuButton::Text => {
                self.marker = Some(marker::Marker::new().expect("failed to create marker"));
            }
            _ => {
                self.marker = None;
            }
        }
    }

    pub fn mouse_update(&mut self, (x, y): (f32, f32)) {
        log::debug!("mouse update: ({}, {})", x, y);
        self.mouse_pixel_coords = (x, y);
        let new_coords = pixels_to_coords((x, y));
        if new_coords != self.mouse_coords {
            self.mouse_coords = new_coords;
            if let Some(marker) = &mut self.marker {
                marker
                    .update(coords_to_pixels(new_coords))
                    .expect("failed to update marker");
            }

            if let State::Modifying {
                guid,
                modification_type,
            } = self.state
            {
                let shape = self.model.get_shape(guid).expect("failed to get shape");
                let (x, y) = coords_to_pixels(self.mouse_coords);
                let event = match modification_type {
                    ModificationType::TL => Event::Modify {
                        guid,
                        data: ShapeUpdate {
                            guid,
                            start: Some(crate::types::Point { x, y }),
                            end: None,
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::TR => Event::Modify {
                        guid,
                        data: ShapeUpdate {
                            guid,
                            start: Some(crate::types::Point {
                                x: shape.start.x,
                                y,
                            }),
                            end: Some(crate::types::Point { x, y: shape.end.y }),
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::BR => Event::Modify {
                        guid,
                        data: ShapeUpdate {
                            guid,
                            start: None,
                            end: Some(crate::types::Point { x, y }),
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::BL => Event::Modify {
                        guid,
                        data: ShapeUpdate {
                            guid,
                            start: Some(crate::types::Point {
                                x,
                                y: shape.start.y,
                            }),
                            end: Some(crate::types::Point { x: shape.end.x, y }),
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::T => Event::Modify {
                        guid,
                        data: ShapeUpdate {
                            guid,
                            start: Some(crate::types::Point {
                                x: shape.start.x,
                                y,
                            }),
                            end: None,
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::R => Event::Modify {
                        guid,
                        data: ShapeUpdate {
                            guid,
                            start: None,
                            end: Some(crate::types::Point { x, y: shape.end.y }),
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::B => Event::Modify {
                        guid,
                        data: ShapeUpdate {
                            guid,
                            start: None,
                            end: Some(crate::types::Point { x: shape.end.x, y }),
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::L => Event::Modify {
                        guid,
                        data: ShapeUpdate {
                            guid,
                            start: Some(crate::types::Point {
                                x,
                                y: shape.start.y,
                            }),
                            end: None,
                            details: None,
                            options: None,
                        },
                    },
                };
                self.model.process_event(event);
                if let Some(selection) = &mut self.selection {
                    selection
                        .update(self.model.get_shape(guid).expect("failed to get shape"))
                        .expect("failed to update selection");
                }
            }
        }
    }

    pub fn mouse_down(&mut self, button: i16) {
        log::debug!("mouse down");
        if button == 2 {
            // TODO: cancel shape creation or modification
            self.state = State::Normal;
            self.set_button_state(MainMenuButton::default());
            return;
        }
        match self.button_state {
            MainMenuButton::Arrow => {
                self.marker = None;
                let (x, y) = coords_to_pixels(self.mouse_coords);
                let event = Event::Add {
                    data: ShapeCreate {
                        guid: None,
                        start: crate::types::Point { x, y },
                        end: crate::types::Point { x, y },
                        details: ShapeDetails::Arrow(ArrowDetails::default()),
                        options: Options::default(),
                    },
                };
                if let Some(guid) = self.model.process_event(event) {
                    self.state = State::Modifying {
                        guid,
                        modification_type: ModificationType::BR,
                    };
                }
            }
            MainMenuButton::Rect => {
                self.marker = None;
                let (x, y) = coords_to_pixels(self.mouse_coords);
                let event = Event::Add {
                    data: ShapeCreate {
                        guid: None,
                        start: crate::types::Point { x, y },
                        end: crate::types::Point { x, y },
                        details: ShapeDetails::Rect(RectDetails::default()),
                        options: Options::default(),
                    },
                };
                if let Some(guid) = self.model.process_event(event) {
                    self.state = State::Modifying {
                        guid,
                        modification_type: ModificationType::BR,
                    };
                }
            }
            _ => {}
        }
    }

    pub fn mouse_up(&mut self) {
        log::debug!("mouse up");
        if let State::Modifying { .. } = self.state {
            self.state = State::Normal;
            self.set_button_state(MainMenuButton::default());
        }
    }

    pub fn modify(&mut self, guid: Guid, modification_type: ModificationType) {
        log::info!("modifying shape: {:?} {:?}", guid, modification_type);
        self.state = State::Modifying {
            guid,
            modification_type,
        };
    }

    pub fn select(&mut self, guid: Guid) {
        log::info!("selecting shape: {:?}", guid);
        self.state = State::Selected { guid };
        let shape = self.model.get_shape(guid).expect("failed to get shape");
        self.selection = Some(Selection::new(shape).expect("failed to create selection"));
    }

    pub fn undo(&mut self) {
        log::info!("undo");
        self.model.undo();
    }

    pub fn redo(&mut self) {
        log::info!("redo");
        self.model.redo();
    }

    pub fn cut(&mut self) {
        log::info!("cut");
        if let State::Selected { guid } = self.state {
            let shape = self.model.get_shape(guid).expect("failed to get shape");
            self.copied_shape = Some(shape.clone());
            self.model.process_event(Event::Remove { guid });
        }
        if self.selection.is_some() {
            self.selection = None;
        }
    }

    pub fn copy(&mut self) {
        log::info!("copy");
        if let State::Selected { guid } = self.state {
            let shape = self.model.get_shape(guid).expect("failed to get shape");
            self.copied_shape = Some(shape.clone());
        }
    }

    pub fn paste(&mut self) {
        log::info!("paste");
        if let Some(shape) = &self.copied_shape {
            let (x, y) = coords_to_pixels(self.mouse_coords);
            let event = Event::Add {
                data: ShapeCreate {
                    guid: None,
                    start: crate::types::Point { x, y },
                    end: crate::types::Point {
                        x: x + shape.end.x - shape.start.x,
                        y: y + shape.end.y - shape.start.y,
                    },
                    details: shape.details.clone(),
                    options: shape.options.clone(),
                },
            };
            self.model.process_event(event);
        }
    }
}
