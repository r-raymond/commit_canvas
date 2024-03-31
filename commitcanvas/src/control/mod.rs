use self::menu::setup;

mod menu;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ButtonState {
    Arrow,
    Rect,
    Text,
    #[default]
    Select,
}

pub struct Control {
    button_state: ButtonState,
}

impl Control {
    pub fn new() -> Control {
        setup().expect("Failed to setup menu");
        Control {
            button_state: ButtonState::Select,
        }
    }

    fn set_button_state(&mut self, button_state: ButtonState) {
        self.button_state = button_state;
    }

    fn get_button_state(&self) -> ButtonState {
        self.button_state
    }
}
