use commitcanvas::control::marker::Marker;
use commitcanvas::control::menu::MainMenuButton;
use commitcanvas::control::selection::Selection;
use commitcanvas::control::Control;
use commitcanvas::control::MouseButton;
use commitcanvas::model::{EventHistory, ShapeConfig, ShapeDetails};
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
fn test_text_creation() {
    let mut control = Control::<TestMarker, TestSelection>::new(Box::new(|_| Ok({})));
    let events = Arc::new(Mutex::new(vec![]));
    let view = TestView {
        events: events.clone(),
    };
    control.add_view(Box::new(view));

    // Set to text mode and create a text box
    control.set_button_state(MainMenuButton::Text);
    control.mouse_update((50.0, 50.0));
    control.mouse_down(MouseButton::Left);
    control.mouse_update((200.0, 100.0));
    control.mouse_up();

    // Verify events were created
    assert!(events.lock().unwrap().len() >= 1, "No events were created");

    // Get the first event and verify it is a text creation event
    let event = &events.lock().unwrap()[0];
    if let EventHistory::Add { config, .. } = event {
        if let ShapeDetails::Text(text_details) = &config.details {
            // Verify it's a text box with default content
            assert_eq!(text_details.content, "", "Text content should be empty by default");
            
            // Verify positions
            assert!(config.start.x <= config.end.x, "Start x should be <= end x");
            assert!(config.start.y <= config.end.y, "Start y should be <= end y");
        } else {
            panic!("Created shape is not a text box");
        }
    } else {
        panic!("Event is not an Add event");
    }
}

#[test]
fn test_text_resize() {
    let mut control = Control::<TestMarker, TestSelection>::new(Box::new(|_| Ok({})));
    let events = Arc::new(Mutex::new(vec![]));
    let view = TestView {
        events: events.clone(),
    };
    control.add_view(Box::new(view));

    // Create a text box
    control.set_button_state(MainMenuButton::Text);
    control.mouse_update((50.0, 50.0));
    control.mouse_down(MouseButton::Left);
    control.mouse_update((150.0, 100.0));
    control.mouse_up();

    // Get the GUID of the created text
    let guid = events.lock().unwrap()[0].guid().unwrap();

    // Resize the text box
    control.select(guid);
    control.modify(guid, commitcanvas::control::ModificationType::BR);
    control.mouse_update((200.0, 150.0));
    control.mouse_up();

    // Check for modification events
    let events_locked = events.lock().unwrap();
    let modification_events: Vec<_> = events_locked.iter()
        .filter(|e| matches!(e, EventHistory::Modify { guid: g, .. } if *g == guid))
        .collect();
    
    assert!(!modification_events.is_empty(), "No modification events found after resize");
}