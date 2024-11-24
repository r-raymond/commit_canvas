use menu::MainMenuUpdate;

use self::menu::MainMenuButton;
use crate::types::{Point, PointPixel};

use crate::view::View;
use crate::{
    model::{
        ArrowDetails, Event, Guid, Model, Options, PartialShapeConfig, RectDetails, ShapeConfig,
        ShapeDetails,
    },
    utils::{coords_to_pixels, pixels_to_coords},
};

pub mod marker;
pub mod menu;
pub mod selection;

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

pub struct Control<M: marker::Marker, S: selection::Selection> {
    button_state: MainMenuButton,
    mouse_pixel_coords: PointPixel,
    mouse_coords: Point<i32>,
    #[allow(dead_code)]
    selection: Option<S>,
    model: Model,
    state: State,
    copied_shape: Option<ShapeConfig>,
    main_menu_update: MainMenuUpdate,
    marker: Option<M>,
}

impl<MARKER: marker::Marker, SELECTION: selection::Selection> Control<MARKER, SELECTION> {
    pub fn new(main_menu_update: MainMenuUpdate) -> Self {
        log::info!("starting contol setup");
        let button_state = MainMenuButton::default();

        let model = Model::new();

        Control {
            button_state,
            mouse_pixel_coords: PointPixel { x: 0.0, y: 0.0 },
            mouse_coords: Point { x: 0, y: 0 },
            marker: None,
            selection: None,
            main_menu_update,
            model,
            state: State::default(),
            copied_shape: None,
        }
    }

    pub fn add_view(&mut self, view: Box<dyn View>) {
        self.model.add_view(view);
    }

    pub fn set_button_state(&mut self, state: MainMenuButton) {
        log::info!("setting button state to {:?}", state);
        self.button_state = state;
        self.selection = None;
        match state {
            MainMenuButton::Arrow => {
                self.marker = Some(MARKER::new().expect("failed to create marker"));
            }
            MainMenuButton::Rect => {
                self.marker = Some(MARKER::new().expect("failed to create marker"));
            }
            MainMenuButton::Text => {
                self.marker = Some(MARKER::new().expect("failed to create marker"));
            }
            _ => {
                self.marker = None;
            }
        }
        self._update_menu();
    }

    fn _update_menu(&self) {
        if let Err(e) = (self.main_menu_update)(self.button_state) {
            log::error!("failed to update menu: {:?}", e);
        }
    }

    pub fn mouse_update(&mut self, (x, y): (f32, f32)) {
        log::debug!("mouse update: ({}, {})", x, y);
        self.mouse_pixel_coords = Point { x, y };
        let new_coords = pixels_to_coords(Point { x, y });
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
                let config = self.model.get_shape(guid).expect("failed to get shape");
                let p = coords_to_pixels(self.mouse_coords);
                let event = match modification_type {
                    ModificationType::TL => Event::Modify {
                        guid,
                        config: PartialShapeConfig {
                            start: Some(p),
                            end: None,
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::TR => Event::Modify {
                        guid,
                        config: PartialShapeConfig {
                            start: Some(PointPixel {
                                x: config.start.x,
                                y: p.y,
                            }),
                            end: Some(PointPixel {
                                x: p.x,
                                y: config.end.y,
                            }),
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::BR => Event::Modify {
                        guid,
                        config: PartialShapeConfig {
                            start: None,
                            end: Some(p),
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::BL => Event::Modify {
                        guid,
                        config: PartialShapeConfig {
                            start: Some(PointPixel {
                                x: p.x,
                                y: config.start.y,
                            }),
                            end: Some(PointPixel {
                                x: config.end.x,
                                y: p.y,
                            }),
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::T => Event::Modify {
                        guid,
                        config: PartialShapeConfig {
                            start: Some(PointPixel {
                                x: config.start.x,
                                y,
                            }),
                            end: None,
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::R => Event::Modify {
                        guid,
                        config: PartialShapeConfig {
                            start: None,
                            end: Some(PointPixel { x, y: config.end.y }),
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::B => Event::Modify {
                        guid,
                        config: PartialShapeConfig {
                            start: None,
                            end: Some(PointPixel { x: config.end.x, y }),
                            details: None,
                            options: None,
                        },
                    },
                    ModificationType::L => Event::Modify {
                        guid,
                        config: PartialShapeConfig {
                            start: Some(PointPixel {
                                x,
                                y: config.start.y,
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
                let mouse = coords_to_pixels(self.mouse_coords);
                let event = Event::Add {
                    guid: None,
                    config: ShapeConfig {
                        start: mouse,
                        end: mouse,
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
                let mouse = coords_to_pixels(self.mouse_coords);
                let event = Event::Add {
                    guid: None,
                    config: ShapeConfig {
                        start: mouse,
                        end: mouse,
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
            self.model.process_event(Event::Checkpoint);
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
        self.selection = Some(SELECTION::new(guid, shape).expect("failed to create selection"));
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
        if let Some(config) = &self.copied_shape {
            let mouse = coords_to_pixels(self.mouse_coords);
            let event = Event::Add {
                guid: None,
                config: ShapeConfig {
                    start: mouse,
                    end: PointPixel {
                        x: mouse.x + config.end.x - config.start.x,
                        y: mouse.y + config.end.y - config.start.y,
                    },
                    details: config.details.clone(),
                    options: config.options.clone(),
                },
            };
            let guid = self
                .model
                .process_event(event)
                .expect("failed to process event");
            let new_shape = self.model.get_shape(guid).expect("failed to get shape");
            self.selection =
                Some(SELECTION::new(guid, new_shape).expect("failed to create selection"));
        }
    }

    pub fn delete(&mut self) {
        log::info!("delete");
        if let State::Selected { guid } = self.state {
            self.model.process_event(Event::Remove { guid });
        }
        if self.selection.is_some() {
            self.selection = None;
        }
    }

    #[cfg(test)]
    pub fn has_selection(&self) -> bool {
        matches!(self.state, State::Selected { .. })
    }
}
