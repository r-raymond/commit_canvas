use wasm_bindgen::JsValue;

mod keyboard;
mod mouse;
mod window;

pub fn setup() -> Result<(), JsValue> {
    keyboard::setup()?;
    mouse::setup()?;
    window::setup()?;
    Ok(())
}
