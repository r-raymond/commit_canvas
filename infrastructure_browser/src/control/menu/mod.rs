mod main;
use wasm_bindgen::JsValue;

pub use main::update;

pub fn setup() -> Result<(), JsValue> {
    log::info!("setting up menus");
    main::setup()?;

    Ok(())
}
