use globals::CONTROL;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

mod control;
mod globals;
mod model;
mod settings;
mod types;
mod utils;
mod view;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    set_panic_hook();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));

    CONTROL.with(|_| {
        log::info!("control initialized");
    });

    Ok(())
}
