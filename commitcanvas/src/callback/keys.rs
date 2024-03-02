use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::state;
use crate::state::editor::EditorMode;

fn key_helper(_event: web_sys::KeyboardEvent, editor_mode: EditorMode) -> Result<(), JsValue> {
    state::STATE.with(|s| -> Result<_, JsValue> {
        let mut state_ref = s.borrow_mut();
        let state = state_ref.as_mut().ok_or("state is None")?;
        state.editor.set_mode(editor_mode)
    })
}

pub fn register(document: &web_sys::Document) -> Result<(), JsValue> {
    {
        let closure = Closure::wrap(Box::new(
            move |event: web_sys::KeyboardEvent| -> Result<(), JsValue> {
                let key = event.key();
                let mode = match key.as_str() {
                    "s" => EditorMode::Normal,
                    "a" => EditorMode::Arrow,
                    "t" => EditorMode::Text { text: None },
                    "r" => EditorMode::Rect { state: None },
                    _ => return Ok(()),
                };
                key_helper(event, mode)
            },
        )
            as Box<dyn FnMut(web_sys::KeyboardEvent) -> Result<(), JsValue>>);
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    Ok(())
}
