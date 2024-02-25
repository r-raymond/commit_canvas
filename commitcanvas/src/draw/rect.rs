use crate::state::STATE;
use rough::Line as RoughLine;
use rough::Point;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub struct Rect {
    pub guid: i32,
    pub start: Point,
    pub end: Point,
    path: web_sys::Element,
    callback: Option<Closure<dyn FnMut(web_sys::MouseEvent) -> Result<(), JsValue>>>,
}

impl Rect {
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
        Ok(Self {
            start,
            end,
            path,
            guid,
            callback: Some(closure),
        })
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
}

impl Drop for Rect {
    fn drop(&mut self) {
        drop(self.callback.take());
        self.path.remove();
    }
}
