use super::point::Point;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    path: web_sys::Element,
}

impl Line {
    pub fn new(
        document: &web_sys::Document,
        svg: &web_sys::SvgElement,
        start: Point,
        end: Point,
        class: &str,
    ) -> Result<Self, JsValue> {
        let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
        path.set_attribute("class", class)?;
        path.set_attribute(
            "d",
            &format!("M {} {} L {} {}", start.x, start.y, end.x, end.y),
        )?;
        svg.append_child(&path)?;
        Ok(Self { start, end, path })
    }

    pub fn update_end(&mut self, end: Point) -> Result<(), JsValue> {
        if self.end == end {
            return Ok(());
        }
        self.end = end;
        self.path.set_attribute(
            "d",
            &format!("M {} {} L {} {}", self.start.x, self.start.y, end.x, end.y),
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn update(&mut self, start: Point, end: Point) -> Result<(), JsValue> {
        if self.start == start && self.end == end {
            return Ok(());
        }
        self.start = start;
        self.end = end;
        self.path.set_attribute(
            "d",
            &format!("M {} {} L {} {}", start.x, start.y, end.x, end.y),
        )?;
        Ok(())
    }

    pub fn set_class(&mut self, class: &str) -> Result<(), JsValue> {
        self.path.set_attribute("class", class)?;
        Ok(())
    }
}

impl Drop for Line {
    fn drop(&mut self) {
        self.path.remove();
    }
}
