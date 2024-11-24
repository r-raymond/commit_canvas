use commitcanvas::view::{Event, View};
use log::info;

pub struct UrlView {}

impl UrlView {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for UrlView {
    fn process_event(
        &mut self,
        _event: Event,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Event::Modify { event } = _event {
            info!("Event: {:?}", event);
        }
        Ok(())
    }
}
