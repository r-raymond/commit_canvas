mod code;
mod event;
mod ui;
mod url;

pub use event::Event;
use wasm_bindgen::JsValue;

pub use ui::UIView;

pub trait View {
    fn process_event(&mut self, event: event::Event) -> Result<(), JsValue>;
}
