use crate::{
    model::{
        ArrowDetails, Event, Guid, Model, Options, RectDetails, ShapeCreate, ShapeDetails,
        ShapeUpdate,
    },
    utils::{coords_to_pixels, pixels_to_coords},
    view::UIView,
};

use self::menu::{setup, update_main_menu, MainMenuButton};

mod callback;
mod marker;
mod menu;

#[derive(Debug, Default)]
enum State {
    #[default]
    Normal,
    Modifying {
        guid: Guid,
    },
}

pub struct Control {
    button_state: MainMenuButton,
    mouse_pixel_coords: (f32, f32),
    mouse_coords: (i32, i32),
    marker: Option<marker::Marker>,
    model: Model,
    state: State,
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
            model,
            state: State::default(),
        }
    }

    pub fn set_button_state(&mut self, state: MainMenuButton) {
        log::info!("setting button state to {:?}", state);
        self.button_state = state;
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

            match self.state {
                State::Modifying { guid } => {
                    let (x, y) = coords_to_pixels(self.mouse_coords);
                    let event = Event::Modify {
                        guid,
                        data: ShapeUpdate {
                            guid,
                            start: None,
                            end: Some(crate::types::Point { x, y }),
                            details: None,
                            options: None,
                        },
                    };
                    self.model.process_event(event);
                }
                _ => {}
            }
        }
    }

    pub fn mouse_down(&mut self) {
        log::info!("mouse down");
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
                    self.state = State::Modifying { guid };
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
                    self.state = State::Modifying { guid };
                }
            }
            _ => {}
        }
    }

    pub fn mouse_up(&mut self) {
        log::info!("mouse up");
        match self.state {
            State::Modifying { .. } => {
                self.state = State::Normal;
                self.set_button_state(MainMenuButton::default());
            }
            _ => {}
        }
    }

    pub fn undo(&mut self) {
        log::info!("undo");
        self.model.undo();
    }

    pub fn redo(&mut self) {
        log::info!("redo");
        self.model.redo();
    }
}
