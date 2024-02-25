use rough::Line as RoughLine;
use rough::Point;
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
        let result = Self {
            start,
            end,
            path: path.clone(),
        };
        path.set_attribute("d", result.render().as_str())?;
        svg.append_child(&path)?;
        Ok(result)
    }

    pub fn update_end(&mut self, end: Point) -> Result<(), JsValue> {
        if self.end == end {
            return Ok(());
        }
        self.end = end;
        self.path.set_attribute("d", self.render().as_str())?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn update(&mut self, start: Point, end: Point) -> Result<(), JsValue> {
        if self.start == start && self.end == end {
            return Ok(());
        }
        self.start = start;
        self.end = end;
        self.path.set_attribute("d", self.render().as_str())?;
        Ok(())
    }

    pub fn set_class(&mut self, class: &str) -> Result<(), JsValue> {
        self.path.set_attribute("class", class)?;
        Ok(())
    }

    fn render(&self) -> String {
        RoughLine::new(self.start, self.end).to_svg_path(10.0)
    }

    pub fn set_id(&mut self, id: i32) -> Result<(), JsValue> {
        self.path.set_id(&format!("{}_line", id));
        Ok(())
    }
}

impl Drop for Line {
    fn drop(&mut self) {
        self.path.remove();
    }
}
