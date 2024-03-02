use super::shape::Shape;
use crate::log;
use crate::state::STATE;
use rough::Line as RoughLine;
use rough::Point;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

struct SelectNode {
    #[allow(dead_code)]
    pub callback: Option<Closure<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>>,
    pub node: web_sys::Element,
}

impl Drop for SelectNode {
    fn drop(&mut self) {
        self.node.remove();
    }
}

impl Default for SelectNode {
    fn default() -> Self {
        Self {
            callback: None,
            node: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")
                .unwrap(),
        }
    }
}

enum ArrowState {
    Normal,
    Removed,
    Selected {
        start: SelectNode,
        end: SelectNode,
    },
    MovingStart {
        start: SelectNode,
        end: SelectNode,
        fallback: Point,
    },
    MovingEnd {
        start: SelectNode,
        end: SelectNode,
        fallback: Point,
    },
}

impl Drop for Arrow {
    fn drop(&mut self) {
        self.path.remove();
    }
}

pub struct Arrow {
    document: web_sys::Document,
    svg: web_sys::SvgElement,
    state: ArrowState,
    path: web_sys::Element,
    pub guid: i32,
    start: Point,
    end: Point,
    #[allow(dead_code)]
    callback: Closure<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>,
}

impl Arrow {
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
        self.svg.append_child(&start)?;
        self.svg.append_child(&end)?;
        let callback_start = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            STATE.with(|s| -> Result<_, JsValue> {
                let mut state_ref = s.borrow_mut();
                let state = state_ref.as_mut().ok_or("state is None")?;
                state.editor.modify(0)?;
                event.prevent_default();
                Ok(())
            })?;
            Ok(())
        })
            as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>);
        start.add_event_listener_with_callback(
            "mousedown",
            callback_start.as_ref().unchecked_ref(),
        )?;
        let callback_end = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            STATE.with(|s| -> Result<_, JsValue> {
                let mut state_ref = s.borrow_mut();
                let state = state_ref.as_mut().ok_or("state is None")?;
                state.editor.modify(1)?;
                event.prevent_default();
                Ok(())
            })?;
            Ok(())
        })
            as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>);
        end.add_event_listener_with_callback("mousedown", callback_end.as_ref().unchecked_ref())?;

        Ok((
            SelectNode {
                callback: Some(callback_start),
                node: start,
            },
            SelectNode {
                callback: Some(callback_end),
                node: end,
            },
        ))
    }

    fn render(&self) -> String {
        RoughLine::new(self.start, self.end).to_svg_path(10.0)
    }
}

impl Shape for Arrow {
    fn new(
        document: &web_sys::Document,
        svg: &web_sys::SvgElement,
        guid: i32,
        start: Point,
    ) -> Result<Self, JsValue>
    where
        Self: Sized,
    {
        let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
        path.set_attribute("class", "cc_arrow")?;
        svg.append_child(&path)?;
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            STATE.with(|s| -> Result<_, JsValue> {
                log(4);
                let mut state_ref = s.borrow_mut();
                let state = state_ref.as_mut().ok_or("state is None")?;
                state.editor.select(guid)?;
                Ok(())
            })?;
            Ok(())
        })
            as Box<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>);
        path.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        Ok(Arrow {
            document: document.clone(),
            svg: svg.clone(),
            state: ArrowState::Normal,
            guid,
            start,
            path,
            end: start,
            callback: closure,
        })
    }

    fn select(&mut self) -> Result<(), JsValue> {
        if let ArrowState::Normal = self.state {
            let (start, end) = self.create_select_nodes()?;
            self.state = ArrowState::Selected { start, end };
        }
        Ok(())
    }

    fn cancel(&mut self) -> Result<(), JsValue> {
        match self.state {
            ArrowState::MovingStart { fallback, .. } => {
                self.update(fallback)?;
                self.path.set_attribute("class", "cc_arrow")?;
            }
            ArrowState::MovingEnd { fallback, .. } => {
                self.update(fallback)?;
                self.path.set_attribute("class", "cc_arrow")?;
            }
            _ => {}
        };
        match &mut self.state {
            ArrowState::Selected { .. } => {
                self.state = ArrowState::Normal;
            }
            ArrowState::MovingStart { start, end, .. } => {
                if self.start == self.end {
                    self.state = ArrowState::Removed;
                } else {
                    self.state = ArrowState::Selected {
                        start: std::mem::take(start),
                        end: std::mem::take(end),
                    };
                }
            }
            ArrowState::MovingEnd { start, end, .. } => {
                if self.start == self.end {
                    self.state = ArrowState::Removed;
                } else {
                    self.state = ArrowState::Selected {
                        start: std::mem::take(start),
                        end: std::mem::take(end),
                    };
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn modify(&mut self, identifier: i32) -> Result<(), JsValue> {
        match &mut self.state {
            ArrowState::Selected { start, end } => {
                if identifier == 0 {
                    self.state = ArrowState::MovingStart {
                        start: std::mem::take(start),
                        end: std::mem::take(end),
                        fallback: self.start,
                    };
                    self.path.set_attribute("class", "cc_arrow_provisional")?;
                } else {
                    self.state = ArrowState::MovingEnd {
                        start: std::mem::take(start),
                        end: std::mem::take(end),
                        fallback: self.end,
                    };
                    self.path.set_attribute("class", "cc_arrow_provisional")?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn commit(&mut self) -> Result<(), JsValue> {
        match &mut self.state {
            ArrowState::MovingStart { start, end, .. } => {
                if self.start == self.end {
                    self.state = ArrowState::Removed;
                } else {
                    self.state = ArrowState::Selected {
                        start: std::mem::take(start),
                        end: std::mem::take(end),
                    };
                    self.path.set_attribute("class", "cc_arrow")?;
                }
            }
            ArrowState::MovingEnd { start, end, .. } => {
                if self.start == self.end {
                    self.state = ArrowState::Removed;
                } else {
                    self.state = ArrowState::Selected {
                        start: std::mem::take(start),
                        end: std::mem::take(end),
                    };
                    self.path.set_attribute("class", "cc_arrow")?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn update(&mut self, point: Point) -> Result<(), JsValue> {
        match &mut self.state {
            ArrowState::MovingStart { start, .. } => {
                self.start = point;
                start.node.set_attribute("cx", &point.x.to_string())?;
                start.node.set_attribute("cy", &point.y.to_string())?;
                self.path.set_attribute("d", self.render().as_str())?;
            }
            ArrowState::MovingEnd { end, .. } => {
                self.end = point;
                end.node.set_attribute("cx", &point.x.to_string())?;
                end.node.set_attribute("cy", &point.y.to_string())?;
                self.path.set_attribute("d", self.render().as_str())?;
            }
            _ => {}
        }
        Ok(())
    }

    fn is_removed(&self) -> bool {
        matches!(self.state, ArrowState::Removed)
    }

    fn is_unselected(&self) -> bool {
        matches!(self.state, ArrowState::Normal)
    }
}
