use crate::draw::shape::Shape;
use crate::state::STATE;
use rough::Line as RoughLine;
use rough::Point;
use select::{CallbackId, SelectState};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

mod select;

enum ArrowState {
    Normal,
    Removed,
    Selected {
        select: SelectState,
    },
    Moving {
        callback_id: CallbackId,
        select: SelectState,
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
        path.set_attribute("marker-end", "url(#cc_arrow_head)")?;
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
            let select = SelectState::new(&self.document, &self.svg, self.start, self.end)?;
            self.state = ArrowState::Selected { select };
        }
        Ok(())
    }

    fn cancel(&mut self) -> Result<(), JsValue> {
        match self.state {
            ArrowState::Moving { fallback, .. } => {
                self.update(fallback)?;
                self.path.set_attribute("class", "cc_arrow")?;
                self.path
                    .set_attribute("marker-end", "url(#cc_arrow_head)")?;
            }
            _ => {}
        };
        match &mut self.state {
            ArrowState::Selected { .. } => {
                self.state = ArrowState::Normal;
            }
            ArrowState::Moving { select, .. } => {
                if self.start == self.end {
                    self.state = ArrowState::Removed;
                } else {
                    self.state = ArrowState::Selected {
                        select: std::mem::take(select),
                    };
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn modify(&mut self, identifier: i32) -> Result<(), JsValue> {
        match &mut self.state {
            ArrowState::Selected { select } => {
                self.state = ArrowState::Moving {
                    select: std::mem::take(select),
                    fallback: self.start,
                    callback_id: identifier.try_into()?,
                };
                self.path.set_attribute("class", "cc_arrow_provisional")?;
                self.path
                    .set_attribute("marker-end", "url(#cc_arrow_head_provisional)")?;
            }
            _ => {}
        }
        Ok(())
    }

    fn commit(&mut self) -> Result<(), JsValue> {
        match &mut self.state {
            ArrowState::Moving { select, .. } => {
                if self.start == self.end {
                    self.state = ArrowState::Removed;
                } else {
                    self.state = ArrowState::Selected {
                        select: std::mem::take(select),
                    };
                    self.path.set_attribute("class", "cc_arrow")?;
                    self.path
                        .set_attribute("marker-end", "url(#cc_arrow_head)")?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn update(&mut self, point: Point) -> Result<(), JsValue> {
        match &mut self.state {
            ArrowState::Moving {
                callback_id,
                select,
                ..
            } => {
                match callback_id {
                    CallbackId::Start => {
                        self.start = point;
                    }
                    CallbackId::End => {
                        self.end = point;
                    }
                }
                select.update(self.start, self.end)?;
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
