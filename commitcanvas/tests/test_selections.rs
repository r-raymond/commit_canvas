use commitcanvas::control::marker::Marker;
use commitcanvas::control::menu::MainMenuButton;
use commitcanvas::control::selection::Selection;
use commitcanvas::control::Control;
use commitcanvas::control::MouseButton;
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

macro_rules! test_selection_remains_after_resize {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut control = Control::<TestMarker, TestSelection>::new(Box::new(|_| Ok({})));
                let events = Arc::new(Mutex::new(vec![]));
                let view = TestView {
                    events: events.clone(),
                };
                control.add_view(Box::new(view));

                control.set_button_state($value);
                control.mouse_update((0.0, 0.0));
                control.mouse_down(MouseButton::Left);
                control.mouse_update((PIXEL_STEP, PIXEL_STEP));
                control.mouse_up();

                assert_eq!(events.lock().unwrap().len(), 3);

                let guid = events.lock().unwrap()[0].guid().unwrap();

                control.select(guid);
                control.modify(guid, commitcanvas::control::ModificationType::T);
                control.mouse_update((2.0 * PIXEL_STEP, 2.0 * PIXEL_STEP));
                control.mouse_up();

                let selected = control.get_selection();

                assert!(selected.is_some());
                assert_eq!(selected.unwrap(), guid);
            }
        )*

    };
}

test_selection_remains_after_resize! {
    test_selection_remains_after_resize_arrow: MainMenuButton::Arrow,
    test_selection_remains_after_resize_rect: MainMenuButton::Rect,
}

macro_rules! test_selected_after_creation {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut control = Control::<TestMarker, TestSelection>::new(Box::new(|_| Ok({})));
                let events = Arc::new(Mutex::new(vec![]));
                let view = TestView {
                    events: events.clone(),
                };
                control.add_view(Box::new(view));

                control.set_button_state($value);
                control.mouse_update((0.0, 0.0));
                control.mouse_down(MouseButton::Left);
                control.mouse_update((PIXEL_STEP, PIXEL_STEP));
                control.mouse_up();

                assert_eq!(events.lock().unwrap().len(), 3);

                let guid = events.lock().unwrap()[0].guid().unwrap();

                let selected = control.get_selection();

                assert!(selected.is_some());
                assert_eq!(selected.unwrap(), guid);
            }
        )*

    };
}

test_selected_after_creation! {
    test_selected_after_creation_arrow: MainMenuButton::Arrow,
    test_selected_after_creation_rect: MainMenuButton::Rect,
}

macro_rules! test_selection_removed_on_random_click {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut control = Control::<TestMarker, TestSelection>::new(Box::new(|_| Ok({})));
                let events = Arc::new(Mutex::new(vec![]));
                let view = TestView {
                    events: events.clone(),
                };
                control.add_view(Box::new(view));

                control.set_button_state($value);
                control.mouse_update((0.0, 0.0));
                control.mouse_down(MouseButton::Left);
                control.mouse_update((PIXEL_STEP, PIXEL_STEP));
                control.mouse_up();

                assert_eq!(events.lock().unwrap().len(), 3);

                let guid = events.lock().unwrap()[0].guid().unwrap();

                let selected = control.get_selection();
                assert!(selected.is_some());
                assert_eq!(selected.unwrap(), guid);

                control.mouse_update((2.0 * PIXEL_STEP, 2.0 * PIXEL_STEP));
                control.mouse_down(MouseButton::Left);
                control.mouse_up();

                let selected = control.get_selection();
                assert!(selected.is_none());
            }
        )*

    };
}

test_selection_removed_on_random_click! {
    test_selection_removed_on_random_click_arrow: MainMenuButton::Arrow,
    test_selection_removed_on_random_click_rect: MainMenuButton::Rect,
}
