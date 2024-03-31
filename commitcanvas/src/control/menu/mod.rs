mod main;
use wasm_bindgen::JsValue;

pub use main::update as update_main_menu;
pub use main::MainMenuButton;

pub fn setup() -> Result<(), JsValue> {
    main::setup()?;
    Ok(())
}
