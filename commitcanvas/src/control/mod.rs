use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

use crate::{
    globals::{CONTROL, SVG},
    utils::{coords_to_pixels, pixels_to_coords},
};

use self::menu::{setup, update_main_menu, MainMenuButton};

mod marker;
mod menu;

pub struct Control {
    button_state: MainMenuButton,
    mouse_pixel_coords: (f32, f32),
    mouse_coords: (i32, i32),
    marker: Option<marker::Marker>,
}

impl Control {
    pub fn new() -> Control {
        log::info!("starting contol setup");
        setup().expect("failed to setup menu");
        let button_state = MainMenuButton::default();
        update_main_menu(button_state).expect("failed to update main menu");

        let mouse_update_closure = Closure::<dyn Fn(MouseEvent)>::new(move |event: MouseEvent| {
            CONTROL.with(|c| {
                let mut control = c.borrow_mut();
                control.mouse_update((event.offset_x() as f32, event.offset_y() as f32));
            });
        });
        SVG.with(|s| s.set_onmousemove(Some(mouse_update_closure.as_ref().unchecked_ref())));
        mouse_update_closure.forget();

        Control {
            button_state: MainMenuButton::default(),
            mouse_pixel_coords: (0., 0.),
            mouse_coords: (0, 0),
            marker: None,
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
        }
    }
}
