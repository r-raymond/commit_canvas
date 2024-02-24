use super::point::Point;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    line: web_sys::Element,
}

impl Line {
    pub fn new(
        document: &web_sys::Document,
        svg: &web_sys::SvgElement,
        start: Point,
        end: Point,
        class: &str,
    ) -> Result<Self, JsValue> {
        let line = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?;
        line.set_attribute("x1", &start.x.to_string())?;
        line.set_attribute("y1", &start.y.to_string())?;
        line.set_attribute("x2", &end.x.to_string())?;
        line.set_attribute("y2", &end.y.to_string())?;
        line.set_attribute("class", class)?;
        svg.append_child(&line)?;
        Ok(Self { start, end, line })
    }

    pub fn update_end(&mut self, end: Point) -> Result<(), JsValue> {
        if self.end == end {
            return Ok(());
        }
        self.end = end;
        self.line.set_attribute("x2", &end.x.to_string())?;
        self.line.set_attribute("y2", &end.y.to_string())?;
        Ok(())
    }

    pub fn update(&mut self, start: Point, end: Point) -> Result<(), JsValue> {
        if self.start == start && self.end == end {
            return Ok(());
        }
        self.start = start;
        self.end = end;
        self.line.set_attribute("x1", &start.x.to_string())?;
        self.line.set_attribute("y1", &start.y.to_string())?;
        self.line.set_attribute("x2", &end.x.to_string())?;
        self.line.set_attribute("y2", &end.y.to_string())?;
        Ok(())
    }

    pub fn set_class(&mut self, class: &str) -> Result<(), JsValue> {
        self.line.set_attribute("class", class)?;
        Ok(())
    }
}

impl Drop for Line {
    fn drop(&mut self) {
        self.line.remove();
    }
}
