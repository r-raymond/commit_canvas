use super::event::Event;

pub struct View {}

#[allow(dead_code)]
impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_event(&mut self, _event: Event) {}
}
