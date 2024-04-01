use wasm_bindgen::{JsCast, JsValue};

use crate::{
    globals::{DOCUMENT, SVG_CONTROL_GROUP},
    model::{Guid, Shape},
};

pub struct Selection {
    pub selected: Guid,
    pub rect: Option<web_sys::SvgElement>,
}

impl Drop for Selection {
    fn drop(&mut self) {
        if let Some(rect) = &self.rect {
            rect.remove();
        }
    }
}

impl Selection {
    pub fn new(shape: Shape) -> Result<Self, JsValue> {
        let rect =
            DOCUMENT.with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect"))?;
        rect.set_id("cc_selection_rect");
        rect.set_attribute("class", "cc_selection_rect")?;
        rect.set_attribute("x", (shape.start.x.min(shape.end.x)).to_string().as_str())?;
        rect.set_attribute("y", (shape.start.y.min(shape.end.y)).to_string().as_str())?;
        rect.set_attribute(
            "width",
            (shape.end.x - shape.start.x).abs().to_string().as_str(),
        )?;
        rect.set_attribute(
            "height",
            (shape.end.y - shape.start.y).abs().to_string().as_str(),
        )?;
        rect.set_attribute("rx", "5")?;

        SVG_CONTROL_GROUP.with(|g| g.append_child(&rect))?;

        Ok(Self {
            selected: shape.guid,
            rect: Some(rect.dyn_into::<web_sys::SvgElement>()?),
        })
    }
}
