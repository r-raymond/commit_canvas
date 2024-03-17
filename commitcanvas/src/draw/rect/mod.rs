use crate::draw::select::{CallbackId, SelectState};
use crate::state::STATE;
use rough::Line as RoughLine;
use rough::Point;
use serde::{ser::SerializeStruct, Serialize};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use super::select::{Colors, LineThickness, Roughness};
use super::Shape;

enum RectState {
    Normal,
    Removed,
    Selected {
        select: SelectState,
    },
    Moving {
        select_id: CallbackId,
        select: SelectState,
        fallback: Point,
    },
}

pub struct Rect {
    document: web_sys::Document,
    svg: web_sys::SvgElement,
    state: RectState,
    path: web_sys::Element,
    rect: web_sys::Element,
    pub guid: i32,
    start: Point,
    end: Point,
    thickness: LineThickness,
    roughness: Roughness,
    fill: Colors,
    #[allow(dead_code)]
    callback: Option<Closure<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>>,
}

impl Drop for Rect {
    fn drop(&mut self) {
        self.path.remove();
    }
}

impl Serialize for Rect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ss = serializer.serialize_struct("Rect", 5)?;
        ss.serialize_field("guid", &self.guid)?;
        ss.serialize_field("start", &self.start)?;
        ss.serialize_field("end", &self.end)?;
        ss.serialize_field("thickness", &self.thickness)?;
        ss.serialize_field("roughness", &self.roughness)?;
        ss.end()
    }
}

impl Shape for Rect {
    fn new(
        document: &web_sys::Document,
        svg: &web_sys::SvgElement,
        guid: i32,
        start: Point,
    ) -> Result<Self, JsValue> {
        let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
        path.set_attribute("class", "cc_rect_provisional")?;
        svg.append_child(&path)?;
        let rect = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
        rect.set_attribute("class", "cc_fill_none stroke-none")?;
        svg.append_child(&rect)?;
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
        Ok(Self {
            document: document.clone(),
            svg: svg.clone(),
            state: RectState::Normal,
            start,
            end: start,
            path,
            rect,
            guid,
            thickness: LineThickness::default(),
            roughness: Roughness::default(),
            fill: Colors::default(),
            callback: Some(closure),
        })
    }

    fn select(&mut self) -> Result<(), JsValue> {
        if let RectState::Normal = self.state {
            let select =
                SelectState::new(&self.document, &self.svg, self.start, self.end, false, true)?;
            self.state = RectState::Selected { select };
        }
        Ok(())
    }

    fn cancel(&mut self) -> Result<(), JsValue> {
        match self.state {
            RectState::Moving { fallback, .. } => {
                self.update(fallback)?;
                self.path.set_attribute("class", "cc_rect")?;
            }
            _ => {}
        }
        match &mut self.state {
            RectState::Selected { .. } => {
                self.state = RectState::Normal;
            }
            RectState::Moving { select, .. } => {
                if self.start == self.end {
                    self.state = RectState::Removed;
                } else {
                    self.state = RectState::Selected {
                        select: std::mem::take(select),
                    };
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn unselect(&mut self) -> Result<(), JsValue> {
        self.state = RectState::Normal;
        Ok(())
    }

    fn modify(&mut self, identifier: CallbackId) -> Result<(), JsValue> {
        match &mut self.state {
            RectState::Selected { select } => match identifier {
                CallbackId::Start => {
                    self.state = RectState::Moving {
                        select_id: identifier,
                        select: std::mem::take(select),
                        fallback: self.start,
                    };
                    self.path.set_attribute("class", "cc_rect_provisional")?;
                }
                CallbackId::End => {
                    self.state = RectState::Moving {
                        select_id: identifier,
                        select: std::mem::take(select),
                        fallback: self.end,
                    };
                    self.path.set_attribute("class", "cc_rect_provisional")?;
                }
                CallbackId::Thickness => {
                    self.thickness.increment();
                    self.path.set_attribute(
                        "stroke-width",
                        f32::from(&self.thickness).to_string().as_str(),
                    )?;
                }
                CallbackId::Roughness => {
                    self.roughness.increment();
                    self.path.set_attribute("d", self.render().as_str())?;
                }
                CallbackId::Fill => {
                    self.fill.increment();
                    let classes = self.rect.class_list();
                    classes.remove_7(
                        "cc_fill_none",
                        "cc_fill_red",
                        "cc_fill_orange",
                        "cc_fill_amber",
                        "cc_fill_yellow",
                        "cc_fill_lime",
                        "cc_fill_green",
                    )?;
                    classes.remove_7(
                        "cc_fill_emerald",
                        "cc_fill_teal",
                        "cc_fill_cyan",
                        "cc_fill_sky",
                        "cc_fill_blue",
                        "cc_fill_indigo",
                        "cc_fill_purple",
                    )?;
                    classes.remove_3("cc_fill_fuchsia", "cc_fill_pink", "cc_fill_rose")?;
                    classes.add_1(self.fill.to_class_name())?;
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn is_removed(&self) -> bool {
        matches!(self.state, RectState::Removed)
    }

    fn is_unselected(&self) -> bool {
        matches!(self.state, RectState::Normal)
    }

    fn commit(&mut self) -> Result<(), JsValue> {
        match &mut self.state {
            RectState::Moving { select, .. } => {
                if self.start == self.end {
                    self.state = RectState::Removed;
                } else {
                    self.state = RectState::Selected {
                        select: std::mem::take(select),
                    };
                    self.path.set_attribute("class", "cc_rect")?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn update(&mut self, point: Point) -> Result<(), JsValue> {
        match &mut self.state {
            RectState::Moving {
                select_id, select, ..
            } => {
                match select_id {
                    CallbackId::Start => {
                        self.start = point;
                    }
                    CallbackId::End => {
                        self.end = point;
                    }
                    _ => {}
                }
                select.update(self.start, self.end)?;
                self.path.set_attribute("d", self.render().as_str())?;
                self.rect
                    .set_attribute("x", self.start.x.to_string().as_str())?;
                self.rect
                    .set_attribute("y", self.start.y.to_string().as_str())?;
                self.rect.set_attribute(
                    "width",
                    (self.end.x - self.start.x).abs().to_string().as_str(),
                )?;
                self.rect.set_attribute(
                    "height",
                    (self.end.y - self.start.y).abs().to_string().as_str(),
                )?;
            }
            _ => {}
        }
        Ok(())
    }

    fn double_click(&mut self) -> Result<(), JsValue> {
        Ok(())
    }

    fn remove(&mut self) -> Result<(), JsValue> {
        self.state = RectState::Removed;
        Ok(())
    }
}

impl Rect {
    fn render(&self) -> String {
        format!(
            "{} {} {} {}",
            RoughLine::new(self.start, Point::new(self.end.x, self.start.y)).to_svg_path(
                (&self.roughness).into(),
                2,
                1.0
            ),
            RoughLine::new(Point::new(self.end.x, self.start.y), self.end).to_svg_path(
                (&self.roughness).into(),
                2,
                1.0
            ),
            RoughLine::new(self.end, Point::new(self.start.x, self.end.y)).to_svg_path(
                (&self.roughness).into(),
                2,
                1.0
            ),
            RoughLine::new(Point::new(self.start.x, self.end.y), self.start).to_svg_path(
                (&self.roughness).into(),
                2,
                1.0
            ),
        )
    }
}
