use crate::state;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

/// Callback for all touch start events on the SVG canvas.
fn touchstart(event: web_sys::TouchEvent) -> Result<(), JsValue> {
    state::STATE.with(|s| -> Result<_, JsValue> {
        let mut state_ref = s.borrow_mut();
        let state = state_ref.as_mut().ok_or("state is None")?;
        state.editor.touchstart(&event)
    })
}

/// Register all callbacks on the svg
pub fn register(svg: &web_sys::SvgElement) -> Result<(), JsValue> {
    {
        let closure = Closure::wrap(
            Box::new(touchstart) as Box<dyn FnMut(web_sys::TouchEvent) -> Result<(), JsValue>>
        );
        svg.add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
