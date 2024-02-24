use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::state;
use crate::state::editor::EditorMode;
use std::collections::HashMap;

fn key_helper(_event: web_sys::KeyboardEvent, editor_mode: EditorMode) -> Result<(), JsValue> {
    state::STATE.with(|s| -> Result<_, JsValue> {
        let mut state_ref = s.borrow_mut();
        let state = state_ref.as_mut().ok_or("state is None")?;
        state.editor.set_mode(editor_mode)
    })
}

pub fn register(document: &web_sys::Document) -> Result<(), JsValue> {
    let mut lookup = HashMap::new();
    lookup.insert("1", EditorMode::Normal);
    lookup.insert("2", EditorMode::Arrow { start: None });
    lookup.insert("3", EditorMode::Square { state: None });
    lookup.insert("4", EditorMode::Text { text: None });

    {
        let closure = Closure::wrap(Box::new(
            move |event: web_sys::KeyboardEvent| -> Result<(), JsValue> {
                let key = event.key();
                if let Some(mode) = lookup.get(key.as_str()) {
                    key_helper(event, mode.clone())
                } else {
                    Ok(())
                }
            },
        )
            as Box<dyn FnMut(web_sys::KeyboardEvent) -> Result<(), JsValue>>);
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    Ok(())
}
