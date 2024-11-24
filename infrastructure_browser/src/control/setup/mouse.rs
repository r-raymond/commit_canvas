use commitcanvas::control::MouseButton;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use crate::globals::{CONTROL, DOCUMENT, SVG};

pub fn setup() -> Result<(), JsValue> {
    let mouse_update_closure =
        Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            CONTROL.with(|c| {
                let mut control = c.borrow_mut();
                control.mouse_update((event.offset_x() as f32, event.offset_y() as f32));
            });
        });
    DOCUMENT.with(|d| d.set_onmousemove(Some(mouse_update_closure.as_ref().unchecked_ref())));
    mouse_update_closure.forget();

    let mouse_down_closure =
        Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            CONTROL.with(|c| {
                let mut control = c.borrow_mut();
                let button = MouseButton::try_from(event.button())
                    .map_err(|_| JsValue::from_str("Invalid mouse button"))
                    .unwrap();
                control.mouse_down(button);
            });
        });
    SVG.with(|s| s.set_onmousedown(Some(mouse_down_closure.as_ref().unchecked_ref())));
    mouse_down_closure.forget();

    let mouse_up_closure =
        Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            CONTROL.with(|c| {
                let mut control = c.borrow_mut();
                control.mouse_up();
            });
        });
    SVG.with(|s| s.set_onmouseup(Some(mouse_up_closure.as_ref().unchecked_ref())));
    mouse_up_closure.forget();

    Ok(())
}
