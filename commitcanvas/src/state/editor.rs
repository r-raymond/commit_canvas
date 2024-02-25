use wasm_bindgen::{JsCast, JsValue};

use super::marker::Marker;
use crate::draw::Line;
use crate::draw::Point;
use crate::draw::Rect;
use crate::state::guid::GuidGenerator;
use std::collections::HashMap;

pub enum EditorMode {
    Normal,
    Arrow {
        state: Option<Line>,
    },
    Rect {
        state: Option<Rect>,
    },
    Text {
        text: Option<web_sys::SvgForeignObjectElement>,
    },
}

pub struct Editor {
    document: web_sys::Document,
    svg: web_sys::SvgElement,
    mode: EditorMode,
    marker: Marker,
    lines: HashMap<i32, Line>,
    rects: HashMap<i32, Rect>,
    guid: GuidGenerator,
}

impl Editor {
    pub fn new(document: web_sys::Document, svg: web_sys::SvgElement) -> Result<Self, JsValue> {
        let mut marker = Marker::new(document.clone(), svg.clone());
        marker.set_marker(false)?;
        Ok(Self {
            document: document.clone(),
            svg: svg.clone(),
            mode: EditorMode::Normal,
            marker,
            lines: HashMap::new(),
            rects: HashMap::new(),
            guid: GuidGenerator::new(),
        })
    }

    pub fn set_mode(&mut self, mode: EditorMode) -> Result<(), JsValue> {
        self.mode = mode;
        match self.mode {
            EditorMode::Normal => {
                self.marker.set_marker(false)?;
                self.set_active_nav_button("selectCanvas")?;
            }
            EditorMode::Arrow { state: _ } => {
                self.marker.set_marker(true)?;
                self.set_active_nav_button("arrowCanvas")?;
            }
            EditorMode::Text { text: _ } => {
                self.marker.set_marker(true)?;
                self.set_active_nav_button("textCanvas")?;
            }
            EditorMode::Rect { state: _ } => {
                self.marker.set_marker(true)?;
                self.set_active_nav_button("rectCanvas")?;
            }
        }
        Ok(())
    }

    fn set_active_nav_button(&self, id: &str) -> Result<(), JsValue> {
        let buttons = self
            .document
            .get_elements_by_class_name("cc_nav_button_selected");
        for i in 0..buttons.length() {
            let button = buttons.item(i).unwrap();
            button
                .dyn_into::<web_sys::HtmlButtonElement>()?
                .class_list()
                .remove_1("cc_nav_button_selected")?;
        }
        let button = self
            .document
            .get_element_by_id(id)
            .expect("No button found")
            .dyn_into::<web_sys::HtmlButtonElement>()?;
        button.class_list().add_1("cc_nav_button_selected")?;
        Ok(())
    }

    pub fn mousedown(&mut self, event: &web_sys::MouseEvent) -> Result<(), JsValue> {
        match &mut self.mode {
            EditorMode::Normal => {}
            EditorMode::Arrow { state } => {
                if let Some(coords) = self.marker.nearest_marker_coords {
                    if state.is_none() {
                        let line = Line::new(
                            &self.document,
                            &self.svg,
                            self.guid.next(),
                            coords.clone(),
                            coords.clone(),
                            "cc_arrow_provisional",
                        )?;
                        *state = Some(line);
                    } else {
                        if event.button() == 2 {
                            state.take();
                            self.set_mode(EditorMode::Normal)?;
                        }
                    }
                }
            }
            EditorMode::Rect { state } => {
                if let Some(coords) = self.marker.nearest_marker_coords {
                    if state.is_none() {
                        let rect = Rect::new(
                            &self.document,
                            &self.svg,
                            coords.clone(),
                            coords.clone(),
                            "cc_rect_provisional",
                        )?;
                        *state = Some(rect);
                    } else {
                        if event.button() == 2 {
                            state.take();
                            self.set_mode(EditorMode::Normal)?;
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn mousemove(&mut self, event: &web_sys::MouseEvent) -> Result<(), JsValue> {
        let coords = Point::new(event.x(), event.y());

        self.marker.set_mouse_coords(coords)?;
        match &mut self.mode {
            EditorMode::Normal => {}
            EditorMode::Arrow { state } => {
                if let (Some(line), Some(coords)) = (state, self.marker.nearest_marker_coords) {
                    line.update_end(coords)?;
                }
            }
            EditorMode::Rect { state } => {
                if let (Some(state), Some(coords)) = (state, self.marker.nearest_marker_coords) {
                    state.update_end(coords)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn mouseup(&mut self, _event: &web_sys::MouseEvent) -> Result<(), JsValue> {
        match &mut self.mode {
            EditorMode::Normal => {}
            EditorMode::Arrow { state } => {
                if let Some(coords) = self.marker.nearest_marker_coords {
                    if let Some(mut line) = state.take() {
                        if coords != line.start {
                            line.set_class("cc_arrow")?;
                            self.lines.insert(line.guid, line);
                        }
                        self.set_mode(EditorMode::Normal)?;
                    }
                }
            }
            EditorMode::Rect { state } => {
                if let Some(mut state) = state.take() {
                    if state.start != state.end {
                        state.set_class("cc_rect")?;
                        let id = self.guid.next();
                        state.set_id(id)?;
                        self.rects.insert(id, state);
                    }
                    self.set_mode(EditorMode::Normal)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn click(&mut self, event: &web_sys::MouseEvent) -> Result<(), JsValue> {
        match &mut self.mode {
            EditorMode::Text { text } => {
                if let Some(coords) = self.marker.nearest_marker_coords {
                    if let Some(fo_wrapper) = text {
                        let textarea = self
                            .document
                            .get_element_by_id("current_textarea")
                            .expect("No textarea found")
                            .dyn_into::<web_sys::HtmlTextAreaElement>()?;
                        let text_node = self
                            .document
                            .create_element_ns(Some("http://www.w3.org/2000/svg"), "text")?;
                        let bounding_box = textarea.get_bounding_client_rect();

                        // If click was on boundig box, then we ignore it
                        if event.x() >= bounding_box.x() as i32
                            && event.x() <= (bounding_box.x() + bounding_box.width()) as i32
                            && event.y() >= bounding_box.y() as i32
                            && event.y() <= (bounding_box.y() + bounding_box.height()) as i32
                        {
                            return Ok(());
                        }

                        let text = textarea.value();
                        text_node.set_attribute("class", "cc_text")?;

                        // Map lines to tspan elements
                        let mut first = true;
                        for line in text.lines() {
                            let tspan = self
                                .document
                                .create_element_ns(Some("http://www.w3.org/2000/svg"), "tspan")?;
                            tspan.set_text_content(Some(line));
                            if first {
                                first = false;
                                tspan.set_attribute(
                                    "y",
                                    (bounding_box.y() + 19.0).to_string().as_str(),
                                )?;
                            } else {
                                tspan.set_attribute("dy", "1.5em")?;
                            }
                            tspan.set_attribute(
                                "x",
                                (bounding_box.x() + 1.0).to_string().as_str(),
                            )?;
                            text_node.append_child(&tspan)?;
                        }

                        fo_wrapper.remove();
                        self.svg.append_child(&text_node)?;
                        self.set_mode(EditorMode::Normal)?;
                    } else {
                        let fo_wrapper = self
                            .document
                            .create_element_ns(Some("http://www.w3.org/2000/svg"), "foreignObject")?
                            .dyn_into::<web_sys::SvgForeignObjectElement>()?;
                        fo_wrapper.set_attribute("x", &coords.x.to_string())?;
                        fo_wrapper.set_attribute("y", &coords.y.to_string())?;
                        fo_wrapper.set_attribute("width", "240")?;
                        fo_wrapper.set_attribute("height", "100")?;
                        fo_wrapper.set_attribute("overflow", "visible")?;
                        //fo_wrapper
                        //    .style()
                        //    .set_property("-webkit-transform", "rotate(20deg)")?;
                        let new_ta = self.document.create_element("textarea")?;
                        new_ta.set_attribute("class", "cc_textarea")?;
                        new_ta.set_id("current_textarea");
                        fo_wrapper.append_child(&new_ta)?;
                        self.svg.append_child(&fo_wrapper)?;
                        *text = Some(fo_wrapper);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn select(&mut self, item: i32) -> Result<(), JsValue> {
        if let Some(line) = self.lines.remove(&item) {
            self.set_mode(EditorMode::Arrow { state: Some(line) })?;
        }
        Ok(())
    }

    pub fn touchstart(&mut self, _event: &web_sys::TouchEvent) -> Result<(), JsValue> {
        Ok(())
    }

    #[allow(dead_code)]
    pub fn touchmove(&mut self, _event: &web_sys::TouchEvent) -> Result<(), JsValue> {
        Ok(())
    }

    #[allow(dead_code)]
    pub fn touchend(&mut self, _event: &web_sys::TouchEvent) -> Result<(), JsValue> {
        Ok(())
    }
}
