use crate::state::STATE;
use rough::Line as RoughLine;
use rough::Point;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

struct SelectNode {
    pub callback: Closure<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>,
    pub node: web_sys::Element,
}

impl Default for SelectNode {
    /// TODO: Make this less crazy. We are adding nodes to the DOM for the type checker
    fn default() -> Self {
        Self {
            callback: Closure::wrap(Box::new(|_event: web_sys::MouseEvent| Ok(()))
                as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>),
            node: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap(),
        }
    }
}

impl Drop for SelectNode {
    fn drop(&mut self) {
        self.node.remove();
    }
}

enum LineState {
    Normal,
    Selected { start: SelectNode, end: SelectNode },
    MovingStart { start: SelectNode, end: SelectNode },
    MovingEnd { start: SelectNode, end: SelectNode },
}

pub struct Line {
    pub guid: i32,
    pub start: Point,
    pub end: Point,
    document: web_sys::Document,
    path: web_sys::Element,
    state: LineState,
    callback: Option<Closure<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>>,
}

impl Line {
    pub fn new(
        document: &web_sys::Document,
        svg: &web_sys::SvgElement,
        guid: i32,
        start: Point,
        end: Point,
        class: &str,
    ) -> Result<Self, JsValue> {
        let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
        path.set_attribute("class", class)?;
        svg.append_child(&path)?;
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            STATE.with(|s| -> Result<_, JsValue> {
                let mut state_ref = s.borrow_mut();
                let state = state_ref.as_mut().ok_or("state is None")?;
                state.editor.select(guid)?;
                Ok(())
            })?;
            Ok(())
        })
            as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>);
        path.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        path.set_attribute("id", &format!("{}_line", guid))?;
        let mut result = Self {
            guid,
            start,
            end,
            document: document.clone(),
            state: LineState::Normal,
            path: path.clone(),
            callback: Some(closure),
        };
        let nodes = result.create_select_nodes()?;
        result.state = LineState::MovingEnd {
            start: nodes.0,
            end: nodes.1,
        };
        Ok(result)
    }

    pub fn update(&mut self, p: Point) -> Result<(), JsValue> {
        match &mut self.state {
            LineState::Normal => {
                return Ok(());
            }
            LineState::Selected { .. } => {
                return Ok(());
            }
            LineState::MovingStart { .. } => {
                if self.start == p {
                    return Ok(());
                }
                self.start = p;
                self.path.set_attribute("d", self.render().as_str())?;
                Ok(())
            }
            LineState::MovingEnd { .. } => {
                if self.end == p {
                    return Ok(());
                }
                self.end = p;
                self.path.set_attribute("d", self.render().as_str())?;
                Ok(())
            }
        }
    }

    fn set_class(&mut self, class: &str) -> Result<(), JsValue> {
        self.path.set_attribute("class", class)?;
        Ok(())
    }

    fn render(&self) -> String {
        RoughLine::new(self.start, self.end).to_svg_path(10.0)
    }

    pub fn mouseup(&mut self, p: Point) -> Result<(), JsValue> {
        match &mut self.state {
            LineState::Normal => Ok::<(), JsValue>(()),
            LineState::Selected { .. } => Ok(()),
            LineState::MovingStart { .. } => {
                self.start = p;
                self.path.set_attribute("d", self.render().as_str())?;
                self.set_class("cc_arrow")?;
                Ok(())
            }
            LineState::MovingEnd { .. } => {
                self.end = p;
                self.path.set_attribute("d", self.render().as_str())?;
                self.set_class("cc_arrow")?;
                Ok(())
            }
        }?;
        self.state = match &mut self.state {
            LineState::Normal => LineState::Normal,
            LineState::Selected { start, end } => LineState::Selected {
                start: std::mem::take(start),
                end: std::mem::take(end),
            },
            LineState::MovingStart { start, end } => LineState::Selected {
                start: std::mem::take(start),
                end: std::mem::take(end),
            },
            LineState::MovingEnd { start, end } => LineState::Selected {
                start: std::mem::take(start),
                end: std::mem::take(end),
            },
        };
        Ok(())
    }

    fn create_select_nodes(&self) -> Result<(SelectNode, SelectNode), JsValue> {
        let start = self
            .document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
        start.set_attribute("cx", &self.start.x.to_string())?;
        start.set_attribute("cy", &self.start.y.to_string())?;
        start.set_attribute("r", "3")?;
        start.set_attribute("class", "cc_arrow_select_node")?;
        let end = self
            .document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
        end.set_attribute("cx", &self.end.x.to_string())?;
        end.set_attribute("cy", &self.end.y.to_string())?;
        end.set_attribute("r", "3")?;
        end.set_attribute("class", "cc_arrow_select_node")?;
        let guid = self.guid;
        Ok((
            SelectNode {
                callback: Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                    STATE.with(|s| -> Result<_, JsValue> {
                        let mut state_ref = s.borrow_mut();
                        let state = state_ref.as_mut().ok_or("state is None")?;
                        state.editor.select(guid)?;
                        Ok(())
                    })?;
                    Ok(())
                })
                    as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>),
                node: start,
            },
            SelectNode {
                callback: Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                    STATE.with(|s| -> Result<_, JsValue> {
                        let mut state_ref = s.borrow_mut();
                        let state = state_ref.as_mut().ok_or("state is None")?;
                        state.editor.select(guid)?;
                        Ok(())
                    })?;
                    Ok(())
                })
                    as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>),
                node: end,
            },
        ))
    }
}

impl Drop for Line {
    fn drop(&mut self) {
        drop(self.callback.take());
        self.path.remove();
    }
}
