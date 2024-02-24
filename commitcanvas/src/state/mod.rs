pub mod editor;
mod marker;

use editor::Editor;
use std::cell::RefCell;
use wasm_bindgen::{JsCast, JsValue};

pub struct State {
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub svg: web_sys::SvgElement,
    pub editor: Editor,
}

impl State {
    pub fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().expect("No window found");
        let document = window.document().expect("No document found");
        let svg = document
            .get_element_by_id("svgCanvas")
            .expect("No svgCanvas found")
            .dyn_into::<web_sys::SvgElement>()?;

        Ok(Self {
            window,
            document: document.clone(),
            svg: svg.clone(),
            editor: Editor::new(document, svg)?,
        })
    }
}

thread_local! {
    pub static STATE: RefCell<Option<State>> = RefCell::new(None);
}
