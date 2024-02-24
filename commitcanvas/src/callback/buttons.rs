use wasm_bindgen::prelude::*;

use crate::state;
use crate::state::editor::EditorMode;

fn click_helper(_event: web_sys::MouseEvent, editor_mode: EditorMode) -> Result<(), JsValue> {
    state::STATE.with(|s| -> Result<_, JsValue> {
        let mut state_ref = s.borrow_mut();
        let state = state_ref.as_mut().ok_or("state is None")?;
        state.editor.set_mode(editor_mode)
    })
}

pub fn register(document: &web_sys::Document) -> Result<(), JsValue> {
    for (id, mode) in vec![
        ("selectCanvas", EditorMode::Normal),
        ("arrowCanvas", EditorMode::Arrow { start: None }),
        ("textCanvas", EditorMode::Text { text: None }),
        ("squareCanvas", EditorMode::Square { state: None }),
    ] {
        let button = document
            .get_element_by_id(id)
            .expect("No button found")
            .dyn_into::<web_sys::HtmlButtonElement>()?;

        {
            let closure = Closure::wrap(Box::new(
                move |event: web_sys::MouseEvent| -> Result<(), JsValue> {
                    click_helper(event, mode.clone())
                },
            )
                as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>);
            button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
    }
    Ok(())
}
