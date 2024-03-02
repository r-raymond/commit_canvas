use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::state;

fn resize(_event: web_sys::Event) -> Result<(), JsValue> {
    state::STATE.with(|s| -> Result<_, JsValue> {
        let mut state_ref = s.borrow_mut();
        let state = state_ref.as_mut().ok_or("state is None")?;
        state.editor.resize()
    })
}

pub fn register(document: &web_sys::Document) -> Result<(), JsValue> {
    {
        let closure = Closure::wrap(
            Box::new(move |_event: web_sys::Event| -> Result<(), JsValue> { resize(_event) })
                as Box<dyn FnMut(web_sys::Event) -> Result<(), JsValue>>,
        );
        document.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    Ok(())
}
