use crate::draw::select::CallbackId;
use erased_serde::Serialize;
use rough::Point;
use wasm_bindgen::JsValue;

pub trait Shape: Serialize {
    fn new(
        document: &web_sys::Document,
        svg: &web_sys::SvgElement,
        guid: i32,
        start: Point,
    ) -> Result<Self, JsValue>
    where
        Self: Sized;

    fn select(&mut self) -> Result<(), JsValue>;

    fn unselect(&mut self) -> Result<(), JsValue>;

    fn cancel(&mut self) -> Result<(), JsValue>;

    fn modify(&mut self, identifier: CallbackId) -> Result<(), JsValue>;

    fn commit(&mut self) -> Result<(), JsValue>;

    fn update(&mut self, point: Point) -> Result<(), JsValue>;

    fn remove(&mut self) -> Result<(), JsValue>;

    fn is_removed(&self) -> bool;

    fn is_unselected(&self) -> bool;

    #[allow(dead_code)]
    fn double_click(&mut self) -> Result<(), JsValue>;
}
