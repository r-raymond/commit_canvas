use wasm_bindgen::prelude::*;

mod buttons;
mod keys;
mod mouse;
mod touch;

/// Register all callback on the svg
pub fn register(document: &web_sys::Document, svg: &web_sys::SvgElement) -> Result<(), JsValue> {
    mouse::register(svg)?;
    touch::register(svg)?;
    buttons::register(document)?;
    keys::register(document)?;
    Ok(())
}
