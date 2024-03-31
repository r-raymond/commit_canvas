use crate::model::Model;

use std::cell::RefCell;
use wasm_bindgen::JsCast;

thread_local! {
    pub static WINDOW: web_sys::Window = web_sys::window().expect("No window found");
    pub static DOCUMENT: web_sys::Document = web_sys::window().expect("No window found").document().expect("No document found");
    pub static SVG: web_sys::SvgElement = DOCUMENT.with(|d| d.get_element_by_id("cc_svg").expect("No svgCanvas found").dyn_into::<web_sys::SvgElement>().expect("Failed to cast to SvgElement"));
    pub static SVG_VIEW_GROUP: web_sys::SvgElement = DOCUMENT.with(|d| d.get_element_by_id("cc_view").expect("No svgCanvas found").dyn_into::<web_sys::SvgElement>().expect("Failed to cast to SvgElement"));
    pub static MODEL: RefCell<Model> = RefCell::new(Model::new());
}