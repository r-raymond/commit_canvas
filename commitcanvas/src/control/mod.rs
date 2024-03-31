use self::menu::{setup, update_main_menu, MainMenuButton};

mod menu;

pub struct Control {
    button_state: MainMenuButton,
}

impl Control {
    pub fn new() -> Control {
        setup().expect("Failed to setup menu");
        let button_state = MainMenuButton::default();
        update_main_menu(button_state).expect("Failed to update main menu");
        Control {
            button_state: MainMenuButton::default(),
        }
    }

    pub fn set_button_state(&mut self, state: MainMenuButton) {
        log::info!("Setting button state to {:?}", state);
        self.button_state = state;
        update_main_menu(state).expect("Failed to update main menu");
    }
}
