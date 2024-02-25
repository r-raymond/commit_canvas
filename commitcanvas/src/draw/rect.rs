use rough::Line as RoughLine;
use rough::Point;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub struct Rect {
    pub start: Point,
    pub end: Point,
    path: web_sys::Element,
}

impl Rect {
    pub fn new(
        document: &web_sys::Document,
        svg: &web_sys::SvgElement,
        start: Point,
        end: Point,
        class: &str,
    ) -> Result<Self, JsValue> {
        let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
        path.set_attribute("class", class)?;
        svg.append_child(&path)?;
        path.set_attribute(
            "d",
            &format!(
                "M {} {} L {} {} L {} {} L {} {} Z",
                start.x, start.y, end.x, start.y, end.x, end.y, start.x, end.y
            ),
        )?;
        Ok(Self { start, end, path })
    }

    pub fn update_end(&mut self, end: Point) -> Result<(), JsValue> {
        if self.end == end {
            return Ok(());
        }
        self.end = end;
        self._update()?;
        Ok(())
    }

    fn _update(&mut self) -> Result<(), JsValue> {
        self.path.set_attribute(
            "d",
            &format!(
                "{} {} {} {}",
                RoughLine::new(self.start, Point::new(self.end.x, self.start.y)).to_svg_path(10.0),
                RoughLine::new(Point::new(self.end.x, self.start.y), self.end).to_svg_path(10.0),
                RoughLine::new(self.end, Point::new(self.start.x, self.end.y)).to_svg_path(10.0),
                RoughLine::new(Point::new(self.start.x, self.end.y), self.start).to_svg_path(10.0),
            ),
        )?;
        Ok(())
    }

    pub fn set_class(&mut self, class: &str) -> Result<(), JsValue> {
        self.path.set_attribute("class", class)?;
        Ok(())
    }

    pub fn set_id(&mut self, id: i32) -> Result<(), JsValue> {
        self.path.set_id(&format!("{}_rect", id));
        Ok(())
    }
}
