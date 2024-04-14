use crate::{
    globals::{CONTROL, DOCUMENT, SVG_VIEW_GROUP},
    model::{Shape, ShapeDetails},
    types::to_identifier,
};
use rough::to_svg_path;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use super::Item;

fn render_path(start: (f32, f32), end: (f32, f32), roughness: f32, rounding: f32) -> String {
    let rounding = rounding
        .min(0.3 * (end.0 - start.0))
        .min(0.3 * (end.1 - start.1));
    format!(
        "{} {} {} {}",
        to_svg_path(start, (end.0, start.1), roughness, 2, 1.0,),
        to_svg_path((end.0, start.1), end, roughness, 2, 1.0,),
        to_svg_path(end, (start.0, end.1), roughness, 2, 1.0,),
        to_svg_path((start.0, end.1), start, roughness, 2, 1.0,),
    )
}

pub fn create_rect(shape: &Shape) -> Result<Item, JsValue> {
    if let ShapeDetails::Rect(d) = &shape.details {
        let svg_path = render_path(
            shape.start.into(),
            shape.end.into(),
            (&shape.options.roughness).into(),
            2.0,
        );

        let path = DOCUMENT
            .with(|document| {
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
            })?
            .dyn_into::<web_sys::SvgElement>()?;
        path.set_attribute("d", &svg_path)?;
        path.set_attribute("class", "cc_rect")?;
        path.set_attribute("filter", "url(#cc_pencil_texture)")?;

        let rect = DOCUMENT
            .with(|document| {
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")
            })?
            .dyn_into::<web_sys::SvgElement>()?;
        rect.class_list()
            .add_2("cc_rect_fill", (&d.background).into())?;

        let group = DOCUMENT
            .with(|document| document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g"))?
            .dyn_into::<web_sys::SvgElement>()?;
        group.set_id(&to_identifier(shape.guid));
        group.append_child(&path)?;
        group.append_child(&rect)?;

        let selector = DOCUMENT
            .with(|document| {
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
            })?
            .dyn_into::<web_sys::SvgElement>()?;
        selector.set_attribute("d", &svg_path)?;
        selector.set_attribute("class", "cc_selector")?;

        let guid = shape.guid;
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

        Ok(Item::Rect {
            group,
            path,
            rect,
            selector,
            selector_closure,
        })
    } else {
        Err(JsValue::from_str("called create_rect with non-rect shape"))
    }
}

pub fn update_rect(shape: &Shape, item: &Item) -> Result<(), JsValue> {
    if let Item::Rect {
        path,
        rect,
        selector,
        ..
    } = item
    {
        if let ShapeDetails::Rect(d) = &shape.details {
            let svg_path = render_path(
                shape.start.into(),
                shape.end.into(),
                (&shape.options.roughness).into(),
                2.0,
            );
            path.set_attribute("d", &svg_path)?;
            selector.set_attribute("d", &svg_path)?;
            let classes = rect.class_list();
            // TODO: Make this more efficient
            classes.remove_7(
                "cc_fill_none",
                "cc_fill_red",
                "cc_fill_orange",
                "cc_fill_amber",
                "cc_fill_yellow",
                "cc_fill_lime",
                "cc_fill_green",
            )?;
            classes.remove_7(
                "cc_fill_emerald",
                "cc_fill_teal",
                "cc_fill_cyan",
                "cc_fill_sky",
                "cc_fill_blue",
                "cc_fill_indigo",
                "cc_fill_purple",
            )?;
            classes.remove_3("cc_fill_fuchsia", "cc_fill_pink", "cc_fill_rose")?;
            classes.add_1((&d.background).into())?;
            Ok(())
        } else {
            Err(JsValue::from_str("called update_rect with non-rect shape"))
        }
    } else {
        Err(JsValue::from_str("Called update_rect with non-rect item"))
    }
}
