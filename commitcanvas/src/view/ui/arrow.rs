use crate::{
    globals::{CONTROL, DOCUMENT, SVG_VIEW_GROUP},
    model::{Guid, ShapeConfig},
    types::to_identifier,
};
use rough::to_svg_path;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use super::Item;

pub fn create_arrow(guid: Guid, config: &ShapeConfig) -> Result<Item, JsValue> {
    let path = DOCUMENT
        .with(|document| document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path"))?
        .dyn_into::<web_sys::SvgPathElement>()?;
    let svg_path = to_svg_path(
        config.start.into(),
        config.end.into(),
        (&config.options.roughness).into(),
        2,
        2.0,
    );
    path.set_attribute("d", &svg_path)?;
    path.set_attribute("class", "cc_arrow")?;
    //path.set_attribute("filter", "url(#cc_pencil_texture)")?;
    path.set_attribute("marker-end", "url(#cc_arrow_head)")?;
    path.set_attribute("stroke-width", (&config.options.thickness).into())?;
    let group = DOCUMENT
        .with(|document| document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g"))?
        .dyn_into::<web_sys::SvgElement>()?;
    group.set_id(&to_identifier(guid));
    group.append_child(&path)?;
    let selector = DOCUMENT
        .with(|document| document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path"))?
        .dyn_into::<web_sys::SvgPathElement>()?;
    selector.set_attribute("d", &svg_path)?;
    selector.set_attribute("class", "cc_selector")?;
    let selector_closure =
        Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
            CONTROL.with(|control| {
                let mut c = control.borrow_mut();
                c.select(guid)
            });
        });
    selector.set_onclick(Some(selector_closure.as_ref().unchecked_ref()));
    group.append_child(&selector)?;
    SVG_VIEW_GROUP.with(|svg| svg.append_child(&group))?;
    Ok(Item::Arrow {
        path,
        selector,
        group,
        selector_closure,
    })
}

pub fn update_arrow(config: &ShapeConfig, item: &Item) -> Result<(), JsValue> {
    if let Item::Arrow { path, selector, .. } = item {
        let svg_path = to_svg_path(
            config.start.into(),
            config.end.into(),
            (&config.options.roughness).into(),
            2,
            2.0,
        );
        path.set_attribute("d", &svg_path)?;
        path.set_attribute("stroke-width", (&config.options.thickness).into())?;
        selector.set_attribute("d", &svg_path)?;
        Ok(())
    } else {
        Err(JsValue::from_str("called update_arrow with non-arrow item"))
    }
}
