use super::event::Event;

pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_event(&mut self, event: Event) {}
}
