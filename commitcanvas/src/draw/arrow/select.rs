use crate::log;
use crate::state::STATE;
use rough::Point;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallbackId {
    Start = 0,
    End = 1,
}

impl TryFrom<i32> for CallbackId {
    type Error = JsValue;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Start),
            1 => Ok(Self::End),
            _ => Err(JsValue::from_str("Invalid CallbackId")),
        }
    }
}

impl From<CallbackId> for i32 {
    fn from(value: CallbackId) -> Self {
        value as i32
    }
}

#[derive(Default)]
pub struct SelectNode {
    #[allow(dead_code)]
    pub callback: Option<Closure<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>>,
    pub node: Option<web_sys::Element>,
}

impl Drop for SelectNode {
    fn drop(&mut self) {
        if let Some(node) = self.node.take() {
            node.remove();
        }
    }
}

impl SelectNode {
    pub fn new(
        document: &web_sys::Document,
        svg: &web_sys::SvgElement,
        x: &str,
        y: &str,
        callback_id: CallbackId,
    ) -> Result<Self, JsValue> {
        let node = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
        node.set_attribute("cx", x)?;
        node.set_attribute("cy", y)?;
        node.set_attribute("r", "5")?;
        node.set_attribute("class", "cc_arrow_select_node")?;
        svg.append_child(&node)?;
        let callback_id = i32::from(callback_id);

        let callback = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            STATE.with(|s| -> Result<_, JsValue> {
                let mut state_ref = s.borrow_mut();
                let state = state_ref.as_mut().ok_or("state is None")?;
                state.editor.modify(callback_id)?;
                event.prevent_default();
                Ok(())
            })?;
            Ok(())
        })
            as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>);
        node.add_event_listener_with_callback("mousedown", callback.as_ref().unchecked_ref())?;

        Ok(Self {
            callback: Some(callback),
            node: Some(node),
        })
    }
}

#[derive(Default)]
pub struct SelectState {
    start: SelectNode,
    end: SelectNode,
    rect: Option<web_sys::Element>,
}

impl Drop for SelectState {
    fn drop(&mut self) {
        if let Some(rect) = self.rect.take() {
            rect.remove();
        }
    }
}

impl SelectState {
    pub fn new(
        document: &web_sys::Document,
        svg: &web_sys::SvgElement,
        start: Point,
        end: Point,
    ) -> Result<Self, JsValue> {
        let rect = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
        rect.set_attribute("x", (std::cmp::min(start.x, end.x)).to_string().as_str())?;
        rect.set_attribute("y", (std::cmp::min(start.y, end.y)).to_string().as_str())?;
        rect.set_attribute("width", (end.x - start.x).abs().to_string().as_str())?;
        rect.set_attribute("height", (end.y - start.y).abs().to_string().as_str())?;
        rect.set_attribute("class", "cc_arrow_select_rect")?;
        svg.append_child(&rect)?;

        let start = SelectNode::new(
            document,
            svg,
            start.x.to_string().as_str(),
            start.y.to_string().as_str(),
            CallbackId::Start,
        )?;
        let end = SelectNode::new(
            document,
            svg,
            end.x.to_string().as_str(),
            end.y.to_string().as_str(),
            CallbackId::End,
        )?;

        Ok(Self {
            start,
            end,
            rect: Some(rect),
        })
    }

    pub fn update(&mut self, start: Point, end: Point) -> Result<(), JsValue> {
        if let Some(node) = &mut self.start.node {
            node.set_attribute("cx", start.x.to_string().as_str())?;
            node.set_attribute("cy", start.y.to_string().as_str())?;
        }
        if let Some(node) = &mut self.end.node {
            node.set_attribute("cx", end.x.to_string().as_str())?;
            node.set_attribute("cy", end.y.to_string().as_str())?;
        }
        if let Some(rect) = &mut self.rect {
            rect.set_attribute("x", (std::cmp::min(start.x, end.x)).to_string().as_str())?;
            rect.set_attribute("y", (std::cmp::min(start.y, end.y)).to_string().as_str())?;
            rect.set_attribute("width", (end.x - start.x).abs().to_string().as_str())?;
            rect.set_attribute("height", (end.y - start.y).abs().to_string().as_str())?;
        }
        Ok(())
    }
}
