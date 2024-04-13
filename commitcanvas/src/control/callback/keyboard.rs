use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use crate::globals::{CONTROL, DOCUMENT};

pub fn setup() -> Result<(), JsValue> {
    let closure = Closure::<dyn FnMut(web_sys::KeyboardEvent) -> Result<(), JsValue>>::new(
        |event: web_sys::KeyboardEvent| {
            let key = event.key();

            // Ctrl + z for undo
            if key == "z" && event.ctrl_key() {
                CONTROL.with(|c| {
                    let mut control = c.borrow_mut();
                    control.undo();
                });
            }

            // Ctrl + y and Ctrl + Shift + z for redo
            if key == "y" && event.ctrl_key() || key == "Z" && event.ctrl_key() {
                CONTROL.with(|c| {
                    let mut control = c.borrow_mut();
                    control.redo();
                });
            }

            if key == "x" && event.ctrl_key() {
                CONTROL.with(|c| {
                    let mut control = c.borrow_mut();
                    control.cut();
                });
            }

            if key == "c" && event.ctrl_key() {
                CONTROL.with(|c| {
                    let mut control = c.borrow_mut();
                    control.copy();
                });
            }

            if key == "v" && event.ctrl_key() {
                CONTROL.with(|c| {
                    let mut control = c.borrow_mut();
                    control.paste();
                });
            }

            Ok(())
        },
    );
    DOCUMENT.with(|d| {
        d.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
    });
    closure.forget();
    Ok(())
}
