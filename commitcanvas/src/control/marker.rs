use crate::globals::{DOCUMENT, SVG_CONTROL_GROUP};
use wasm_bindgen::JsValue;

pub struct Marker {
    marker: web_sys::Element,
}

impl Drop for Marker {
    fn drop(&mut self) {
        self.marker.remove();
    }
}

impl Marker {
    pub fn new() -> Result<Self, JsValue> {
        let marker =
            DOCUMENT.with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?;
        marker.set_attribute("r", "3")?;
        marker.set_attribute("class", "cc_nearest_marker")?;
        SVG_CONTROL_GROUP.with(|svg| svg.append_child(&marker))?;
        Ok(Self { marker })
    }

    pub fn update(&self, (x, y): (f32, f32)) -> Result<(), JsValue> {
        self.marker.set_attribute("cx", &x.to_string())?;
        self.marker.set_attribute("cy", &y.to_string())?;
        Ok(())
    }
}
