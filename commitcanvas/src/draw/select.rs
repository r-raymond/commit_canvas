use crate::state::PIXEL_STEP;
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

struct ContextMenu {
    menu: web_sys::SvgForeignObjectElement,
}

impl Drop for ContextMenu {
    fn drop(&mut self) {
        self.menu.remove();
    }
}

#[derive(Default)]
pub struct SelectState {
    start: SelectNode,
    end: SelectNode,
    rect: Option<web_sys::Element>,
    context_menu: Option<ContextMenu>,
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
        with_rect: bool,
    ) -> Result<Self, JsValue> {
        let rect = if with_rect {
            let rect = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
            rect.set_attribute("x", (std::cmp::min(start.x, end.x)).to_string().as_str())?;
            rect.set_attribute("y", (std::cmp::min(start.y, end.y)).to_string().as_str())?;
            rect.set_attribute("width", (end.x - start.x).abs().to_string().as_str())?;
            rect.set_attribute("height", (end.y - start.y).abs().to_string().as_str())?;
            rect.set_attribute("rx", "5")?;
            rect.set_attribute("class", "cc_arrow_select_rect")?;
            svg.append_child(&rect)?;
            Some(rect)
        } else {
            None
        };

        let context_menu = {
            let menu =
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "foreignObject")?;
            menu.set_attribute(
                "x",
                (std::cmp::max(start.x, end.x) + 2 * PIXEL_STEP)
                    .to_string()
                    .as_str(),
            )?;
            menu.set_attribute("y", std::cmp::min(start.y, end.y).to_string().as_str())?;
            menu.set_attribute("width", (8 * PIXEL_STEP).to_string().as_str())?;
            menu.set_attribute("height", (32 * PIXEL_STEP).to_string().as_str())?;
            menu.set_attribute("class", "cc_context_menu")?;
            let div = document.create_element("div")?;
            div.set_attribute("class", "cc_context_menu_div")?;
            menu.append_child(&div)?;
            //svg.set_attribute("xmlns", "http://www.w3.org/2000/svg")?;
            //svg.set_attribute("height", "24px")?;
            //svg.set_attribute("viewBox", "0 0 24 24")?;
            //svg.set_attribute("width", "24px")?;
            //svg.set_attribute("fill", "#000000")?;
            //let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
            //path.set_attribute(
            //    "d",
            //    "M3 17h18v-2H3v2zm0 3h18v-1H3v1zm0-7h18v-3H3v3zm0-9v4h18V4H3z",
            //)?;
            //path.set_attribute("fill", "none")?;
            //svg.append_child(&path)?;
            //button_1.append_child(&svg)?;

            let button_1 = document.create_element("button")?;
            button_1.set_text_content(Some("Select"));
            button_1.set_attribute("class", "cc_context_menu_button cc_context_menu_button_top")?;
            div.append_child(&button_1)?;
            let button_2 = document.create_element("button")?;
            button_2.set_text_content(Some("Copy"));
            button_2.set_attribute("class", "cc_context_menu_button")?;
            div.append_child(&button_2)?;
            let button_3 = document.create_element("button")?;
            button_3.set_text_content(Some("Cut"));
            button_3.set_attribute("class", "cc_context_menu_button")?;
            div.append_child(&button_3)?;
            let button_4 = document.create_element("button")?;
            button_4.set_text_content(Some("Paste"));
            button_4.set_attribute(
                "class",
                "cc_context_menu_button cc_context_menu_button_bottom",
            )?;
            div.append_child(&button_4)?;
            svg.append_child(&menu)?;
            let fo = menu.dyn_into::<web_sys::SvgForeignObjectElement>()?;
            Some(ContextMenu { menu: fo })
        };

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
            rect,
            context_menu,
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
        if let Some(menu) = &mut self.context_menu {
            menu.menu.set_attribute(
                "x",
                (std::cmp::max(start.x, end.x) + 2 * PIXEL_STEP)
                    .to_string()
                    .as_str(),
            )?;
            menu.menu
                .set_attribute("y", std::cmp::min(start.y, end.y).to_string().as_str())?;
        }
        Ok(())
    }
}
