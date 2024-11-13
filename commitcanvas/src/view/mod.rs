mod event;
use std::error::Error;

pub use event::Event;

pub trait View {
    fn process_event(&mut self, event: event::Event) -> Result<(), Box<dyn Error + Send + Sync>>;
}
