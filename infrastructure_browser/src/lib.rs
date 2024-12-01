mod control;
mod globals;
mod view;
use globals::CONTROL;
use wasm_bindgen::prelude::*;

mod utils;
use utils::set_panic_hook;

/// Initializes the WebAssembly module.
///
/// This function is called when the WebAssembly module is loaded and executed.
/// It sets up the panic hook for better error messages, initializes the logger,
/// and logs a message indicating that the control has been initialized. It sets
/// up the global variables required.
///
/// # Returns
///
/// * `Result<(), JsValue>` - A result indicating success or an error.
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    set_panic_hook();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));

    CONTROL.with(|_| {
        log::info!("control initialized");
    });

    log::info!("calling setups");
    control::menu::setup()?;
    control::setup::setup()?;
    let uiview = SVG.with(|s| view::ui::UiView::setup(s))?;
    let urlview = view::url::UrlView::new();
    CONTROL.with(|c| {
        let mut control = c.borrow_mut();
        control.add_view(Box::new(uiview));
        control.add_view(Box::new(urlview));
    });

    Ok(())
}
