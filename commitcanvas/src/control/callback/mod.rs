use wasm_bindgen::JsValue;

mod keyboard;
mod mouse;

pub fn setup() -> Result<(), JsValue> {
    keyboard::setup()?;
    mouse::setup()?;
    Ok(())
}
