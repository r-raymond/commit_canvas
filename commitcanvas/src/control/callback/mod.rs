use wasm_bindgen::JsValue;

mod keyboard;

pub fn setup() -> Result<(), JsValue> {
    keyboard::setup()?;
    Ok(())
}
