use rough::geometry::Vector;
use wasm_bindgen::{JsCast, JsValue};

use super::marker::Marker;
use crate::draw::select::CallbackId;
use crate::draw::Arrow;
use crate::draw::Point;
use crate::draw::Rect;
use crate::draw::Shape;
use crate::state::guid::GuidGenerator;
use std::collections::HashMap;

pub enum EditorMode {
    Normal,
    Panning {
        start: Point,
    },
    Arrow,
    Rect,
    Selected {
        item: i32,
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
    shapes: HashMap<i32, Box<dyn Shape>>,
    guid: GuidGenerator,
    offset: Vector,
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
            shapes: HashMap::new(),
            guid: GuidGenerator::new(),
            offset: Vector::new(0f32, 0f32),
        })
    }

    pub fn set_mode(&mut self, mode: EditorMode) -> Result<(), JsValue> {
        match self.mode {
            EditorMode::Selected { item } => {
                if let Some(shape) = self.shapes.get_mut(&item) {
                    shape.unselect()?;
                }
            }
            _ => {}
        }
        self.mode = mode;
        match self.mode {
            EditorMode::Normal => {
                self.svg.set_attribute("cursor", "normal")?;
                self.marker.set_marker(false)?;
                self.set_active_nav_button(Some("selectCanvas"))?;
            }
            EditorMode::Arrow => {
                self.svg.set_attribute("cursor", "normal")?;
                self.marker.set_marker(true)?;
                self.set_active_nav_button(Some("arrowCanvas"))?;
            }
            EditorMode::Rect => {
                self.svg.set_attribute("cursor", "normal")?;
                self.marker.set_marker(true)?;
                self.set_active_nav_button(Some("rectCanvas"))?;
            }
            EditorMode::Selected { item: _ } => {
                self.svg.set_attribute("cursor", "normal")?;
                self.marker.set_marker(false)?;
                self.set_active_nav_button(None)?;
            }
            EditorMode::Panning { .. } => {
                self.svg.set_attribute("cursor", "grabbing")?;
                self.marker.set_marker(false)?;
                self.set_active_nav_button(None)?;
            }
            EditorMode::Text { text: _ } => {
                self.svg.set_attribute("cursor", "normal")?;
                self.marker.set_marker(true)?;
                self.set_active_nav_button(Some("textCanvas"))?;
            }
        }
        Ok(())
    }

    fn set_active_nav_button(&self, mid: Option<&str>) -> Result<(), JsValue> {
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
        if let Some(id) = mid {
            let button = self
                .document
                .get_element_by_id(id)
                .expect("No button found")
                .dyn_into::<web_sys::HtmlButtonElement>()?;
            button.class_list().add_1("cc_nav_button_selected")?;
        }
        Ok(())
    }

    pub fn mousedown(&mut self, event: &web_sys::MouseEvent) -> Result<(), JsValue> {
        match &mut self.mode {
            EditorMode::Normal => {
                let coords = Point::new(event.x(), event.y());
                if event.button() == 1 {
                    self.set_mode(EditorMode::Panning { start: coords })?;
                }
            }
            EditorMode::Selected { item } => {
                if let Some(shape) = self.shapes.get_mut(item) {
                    if event.button() == 2 {
                        shape.cancel()?;
                        if shape.is_removed() {
                            self.shapes.remove(item);
                            self.set_mode(EditorMode::Normal)?;
                        } else if shape.is_unselected() {
                            self.set_mode(EditorMode::Normal)?;
                        }
                    }
                }
            }
            EditorMode::Arrow => {
                if let Some(coords) = self.marker.nearest_marker_coords {
                    let mut shape =
                        Arrow::new(&self.document, &self.svg, self.guid.next(), coords.clone())?;
                    shape.select()?;
                    shape.modify(CallbackId::End)?;
                    self.set_mode(EditorMode::Selected { item: shape.guid })?;
                    self.shapes.insert(shape.guid, Box::new(shape));
                }
            }
            EditorMode::Rect => {
                if let Some(coords) = self.marker.nearest_marker_coords {
                    let mut shape =
                        Rect::new(&self.document, &self.svg, self.guid.next(), coords.clone())?;
                    shape.select()?;
                    shape.modify(CallbackId::End)?;
                    self.set_mode(EditorMode::Selected { item: shape.guid })?;
                    self.shapes.insert(shape.guid, Box::new(shape));
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
            EditorMode::Panning { start } => {
                self.offset = *start - &coords;
                let bb = self.svg.get_bounding_client_rect();
                self.svg.set_attribute(
                    "viewBox",
                    &format!(
                        "{} {} {} {}",
                        self.offset.x,
                        self.offset.y,
                        bb.width() as f32,
                        bb.height() as f32
                    ),
                )?;
            }
            EditorMode::Selected { item } => {
                if let Some(shape) = self.shapes.get_mut(item) {
                    if let Some(coords) = self.marker.nearest_marker_coords {
                        shape.update(coords)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn mouseup(&mut self, _event: &web_sys::MouseEvent) -> Result<(), JsValue> {
        match &mut self.mode {
            EditorMode::Normal => {}
            EditorMode::Panning { .. } => {
                self.set_mode(EditorMode::Normal)?;
                self.marker.offset = self.offset.clone();
            }
            EditorMode::Selected { item } => {
                if let Some(shape) = self.shapes.get_mut(item) {
                    shape.commit()?;
                    if shape.is_removed() {
                        self.shapes.remove(item);
                        self.set_mode(EditorMode::Normal)?;
                    } else if shape.is_unselected() {
                        self.set_mode(EditorMode::Normal)?;
                    }
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
        if let Some(shape) = self.shapes.get_mut(&item) {
            shape.select()?;
            self.set_mode(EditorMode::Selected { item })?;
        }
        Ok(())
    }

    pub fn modify(&mut self, identifier: CallbackId) -> Result<(), JsValue> {
        if let EditorMode::Selected { item } = self.mode {
            self.shapes.get_mut(&item).unwrap().modify(identifier)?;
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

    pub fn delete(&mut self) -> Result<(), JsValue> {
        if let EditorMode::Selected { item } = self.mode {
            if let Some(shape) = self.shapes.get_mut(&item) {
                shape.remove()?;
            }
        }
        Ok(())
    }
}
