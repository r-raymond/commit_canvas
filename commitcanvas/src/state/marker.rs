use crate::log;
use rough::geometry::Vector;
use wasm_bindgen::JsValue;

use crate::draw::Point;

const PIXEL_STEP: i32 = 6;

pub struct Marker {
    document: web_sys::Document,
    svg: web_sys::SvgElement,
    pub nearest_marker: Option<web_sys::Element>,
    pub nearest_marker_coords: Option<Point>,
    mouse_coords: Option<Point>,
    marker_on: bool,
    pub offset: Vector,
}

impl Marker {
    pub fn new(document: web_sys::Document, svg: web_sys::SvgElement) -> Self {
        Self {
            document,
            svg,
            nearest_marker: None,
            nearest_marker_coords: None,
            mouse_coords: None,
            marker_on: false,
            offset: Vector::new(0.0, 0.0),
        }
    }

    pub fn set_mouse_coords(&mut self, coords: Point) -> Result<(), JsValue> {
        log(self.offset.x as usize);
        log(coords.x as usize);
        self.mouse_coords = Some(coords);
        let coords = coords + &self.offset;
        log(coords.x as usize);
        self.nearest_marker_coords = Some(Point::new(
            ((coords.x - PIXEL_STEP) as f32 / 12.0).round() as i32 * PIXEL_STEP * 2 + PIXEL_STEP,
            ((coords.y - PIXEL_STEP) as f32 / 12.0).round() as i32 * PIXEL_STEP * 2 + PIXEL_STEP,
        ));

        if self.marker_on {
            if self.nearest_marker.is_none() {
                self.create_marker()?;
            }
            self.update_marker()?;
        }

        Ok(())
    }

    pub fn set_marker(&mut self, on: bool) -> Result<(), JsValue> {
        self.marker_on = on;
        if !on {
            if let Some(marker) = &self.nearest_marker {
                self.svg.remove_child(marker)?;
            }
            self.nearest_marker = None;
        }
        Ok(())
    }

    fn create_marker(&mut self) -> Result<(), JsValue> {
        let marker = self
            .document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
        marker.set_attribute("r", "3")?;
        marker.set_id("cc_nearest_marker");
        self.svg.append_child(&marker)?;
        self.nearest_marker = Some(marker);
        Ok(())
    }

    fn update_marker(&mut self) -> Result<(), JsValue> {
        if let Some(coord) = self.nearest_marker_coords {
            if let Some(marker) = &self.nearest_marker {
                marker.set_attribute("cx", &coord.x.to_string())?;
                marker.set_attribute("cy", &coord.y.to_string())?;
            }
        }
        Ok(())
    }
}
