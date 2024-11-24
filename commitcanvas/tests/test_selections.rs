use commitcanvas::control::marker::Marker;
use commitcanvas::control::menu::MainMenuButton;
use commitcanvas::control::selection::Selection;
use commitcanvas::control::Control;
use commitcanvas::model::{EventHistory, ShapeConfig};
use commitcanvas::settings::PIXEL_STEP;
use commitcanvas::types::{Guid, PointPixel};
use commitcanvas::view::{Event, View};
use std::error::Error;
use std::sync::{Arc, Mutex};

struct TestMarker;

impl Marker for TestMarker {
    fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self)
    }

    #[allow(unused_variables)]
    fn update(&self, p: PointPixel) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}

struct TestSelection;

impl Selection for TestSelection {
    #[allow(unused_variables)]
    fn new(guid: Guid, config: &ShapeConfig) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self)
    }

    #[allow(unused_variables)]
    fn update(&mut self, config: &ShapeConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}

struct TestView {
    events: Arc<Mutex<Vec<EventHistory>>>,
}

impl View for TestView {
    #[allow(unused_variables)]
    fn process_event(&mut self, event: Event) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Event::Modify { event } = event {
            self.events.lock().unwrap().push(event);
        }
        Ok(())
    }
}

#[test]
fn test_selection_remains_after_resize() {
    let mut control = Control::<TestMarker, TestSelection>::new(Box::new(|_| Ok({})));
    let events = Arc::new(Mutex::new(vec![]));
    let view = TestView {
        events: events.clone(),
    };
    control.add_view(Box::new(view));

    control.set_button_state(MainMenuButton::Arrow);
    control.mouse_update((0.0, 0.0));
    control.mouse_down(1);
    control.mouse_update((PIXEL_STEP, PIXEL_STEP));
    control.mouse_up();

    assert_eq!(events.lock().unwrap().len(), 3);
}
