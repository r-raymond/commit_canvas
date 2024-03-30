mod event;
mod url;

pub use event::Event;

pub trait View {
    fn process_event(&mut self, event: event::Event) -> ();
}
