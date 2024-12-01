use commitcanvas::view::{Event, View};

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
        if let Event::Modify { .. } = _event {}
        Ok(())
    }
}
