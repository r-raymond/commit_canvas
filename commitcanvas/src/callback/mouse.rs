use crate::state;
use wasm_bindgen::prelude::*;

/// Callback for all mouse click events on the SVG canvas.
fn click(event: web_sys::MouseEvent) -> Result<(), JsValue> {
    state::STATE.with(|s| -> Result<_, JsValue> {
        let mut state_ref = s.borrow_mut();
        let state = state_ref.as_mut().ok_or("state is None")?;
        state.editor.click(&event)
    })
}

/// Callback for all movement events on the SVG canvas.
fn mousemove(event: web_sys::MouseEvent) -> Result<(), JsValue> {
    state::STATE.with(|s| -> Result<_, JsValue> {
        let mut state_ref = s.borrow_mut();
        let state = state_ref.as_mut().ok_or("state is None")?;
        state.editor.mousemove(&event)
    })
}

/// Callback for all mousedown events on the SVG canvas.
fn mousedown(event: web_sys::MouseEvent) -> Result<(), JsValue> {
    state::STATE.with(|s| -> Result<_, JsValue> {
        let mut state_ref = s.borrow_mut();
        let state = state_ref.as_mut().ok_or("state is None")?;
        state.editor.mousedown(&event)
    })
}

/// Callback for all mouseup events on the SVG canvas.
fn mouseup(event: web_sys::MouseEvent) -> Result<(), JsValue> {
    state::STATE.with(|s| -> Result<_, JsValue> {
        let mut state_ref = s.borrow_mut();
        let state = state_ref.as_mut().ok_or("state is None")?;
        state.editor.mouseup(&event)
    })
}

/// Register all callback on the svg
pub fn register(svg: &web_sys::SvgElement) -> Result<(), JsValue> {
    {
        let closure = Closure::wrap(
            Box::new(click) as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>
        );
        svg.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let closure = Closure::wrap(
            Box::new(mousemove) as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>
        );
        svg.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let closure = Closure::wrap(
            Box::new(mousedown) as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>
        );
        svg.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let closure = Closure::wrap(
            Box::new(mouseup) as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>
        );
        svg.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
