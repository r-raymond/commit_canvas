use globals::CONTROL;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

mod callback;
mod control;
mod draw;
mod globals;
mod model;
mod settings;
mod state;
mod types;
mod utils;
mod view;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    set_panic_hook();
    wasm_logger::init(wasm_logger::Config::default());

    CONTROL.with(|c| {});

    //let state = state::State::new()?;
    //state::STATE.with(|s| *s.borrow_mut() = Some(state));
    //state::STATE.with(|s| -> Result<_, JsValue> {
    //    let state_ref = s.borrow();
    //    let state = state_ref.as_ref().ok_or("state is None")?;
    //    callback::register(&state.document, &state.svg)
    //})?;

    Ok(())
}
