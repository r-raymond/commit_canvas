use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

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
    for id in vec!["selectCanvas", "arrowCanvas", "textCanvas", "rectCanvas"] {
        let button = document
            .get_element_by_id(id)
            .expect("No button found")
            .dyn_into::<web_sys::HtmlButtonElement>()?;

        {
            let closure = Closure::wrap(Box::new(
                move |event: web_sys::MouseEvent| -> Result<(), JsValue> {
                    let mode = match id {
                        "selectCanvas" => EditorMode::Normal,
                        "arrowCanvas" => EditorMode::Arrow,
                        "textCanvas" => EditorMode::Text { text: None },
                        "rectCanvas" => EditorMode::Rect { state: None },
                        _ => panic!("Unknown button id"),
                    };
                    click_helper(event, mode)
                },
            )
                as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>);
            button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
    }
    Ok(())
}
