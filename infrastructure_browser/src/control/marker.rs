use std::error::Error;

use crate::globals::{DOCUMENT, SVG_CONTROL_GROUP};
use crate::utils::to_error;

use commitcanvas::control::marker::Marker as MarkerInterface;
use commitcanvas::types::PointPixel;

pub struct Marker {
    marker: web_sys::Element,
}

impl Drop for Marker {
    fn drop(&mut self) {
        self.marker.remove();
    }
}

impl MarkerInterface for Marker {
    fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let marker = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))
            .map_err(to_error)?;
        marker.set_attribute("r", "3").map_err(to_error)?;
        marker
            .set_attribute("class", "cc_nearest_marker")
            .map_err(to_error)?;
        SVG_CONTROL_GROUP
            .with(|svg| svg.append_child(&marker))
            .map_err(to_error)?;
        Ok(Self { marker })
    }

    fn update(&self, p: PointPixel) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.marker
            .set_attribute("cx", &p.x.to_string())
            .map_err(to_error)?;
        self.marker
            .set_attribute("cy", &p.y.to_string())
            .map_err(to_error)?;
        Ok(())
    }
}
