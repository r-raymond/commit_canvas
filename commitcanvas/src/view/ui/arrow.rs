use crate::{globals::DOCUMENT, model::Shape};
use rough::to_svg_path;
use wasm_bindgen::JsValue;

pub fn create_arrow(shape: &Shape) -> Result<web_sys::Element, JsValue> {
    let path = DOCUMENT.with(|document| document.create_element("path"))?;
    let svg_path = to_svg_path(
        shape.start.into(),
        shape.end.into(),
        (&shape.options.roughness).into(),
        2,
        2.0,
    );
    path.set_attribute("d", &svg_path)?;
    path.set_attribute("class", "cc_arrow")?;
    path.set_attribute("filter", "url(#cc_pencil_texture)")?;
    path.set_attribute("marker-end", "url(#cc_arrow_head)")?;
    path.set_attribute("stroke-width", (&shape.options.thickness).into())?;
    Ok(path)
}
