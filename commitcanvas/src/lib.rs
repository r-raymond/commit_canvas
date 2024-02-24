use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

mod callback;
mod draw;
mod state;
mod utils;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: usize);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(format!("Hello {}!", name).as_str());
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let state = state::State::new()?;
    state::STATE.with(|s| *s.borrow_mut() = Some(state));
    set_panic_hook();

    state::STATE.with(|s| -> Result<_, JsValue> {
        let state_ref = s.borrow();
        let state = state_ref.as_ref().ok_or("state is None")?;
        callback::register(&state.document, &state.svg)
    })?;

    Ok(())
}
