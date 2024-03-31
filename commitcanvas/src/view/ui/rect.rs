use crate::{
    globals::{DOCUMENT, SVG_VIEW_GROUP},
    model::{Shape, ShapeDetails},
    types::to_identifier,
};
use rough::to_svg_path;
use wasm_bindgen::JsValue;

use super::Item;

pub fn create_rect(shape: &Shape) -> Result<Item, JsValue> {
    if let ShapeDetails::Rect(d) = &shape.details {
        let svg_path = format!(
            "{} {} {} {}",
            to_svg_path(
                shape.start.into(),
                (shape.end.x, shape.start.y),
                (&shape.options.roughness).into(),
                2,
                1.0,
            ),
            to_svg_path(
                (shape.end.x, shape.start.y),
                shape.end.into(),
                (&shape.options.roughness).into(),
                2,
                1.0,
            ),
            to_svg_path(
                shape.end.into(),
                (shape.start.x, shape.end.y),
                (&shape.options.roughness).into(),
                2,
                1.0,
            ),
            to_svg_path(
                (shape.start.x, shape.end.y),
                shape.start.into(),
                (&shape.options.roughness).into(),
                2,
                1.0,
            ),
        );

        let path = DOCUMENT.with(|document| {
            document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
        })?;
        path.set_attribute("d", &svg_path)?;
        path.set_attribute("class", "cc_rect")?;
        path.set_attribute("filter", "url(#cc_pencil_texture)")?;
        let rect = DOCUMENT.with(|document| {
            document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")
        })?;
        rect.class_list()
            .add_2("cc_rect_fill", (&d.background).into())?;
        let group = DOCUMENT
            .with(|document| document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g"))?;
        group.set_id(&to_identifier(shape.guid));
        group.append_child(&path)?;
        group.append_child(&rect)?;
        SVG_VIEW_GROUP.with(|svg| svg.append_child(&group))?;
        Ok(Item::Rect { path, rect })
    } else {
        Err(JsValue::from_str("Called create_rect with non-rect shape"))
    }
}

pub fn update_rect(shape: &Shape, item: &Item) -> Result<(), JsValue> {
    if let Item::Rect { path, rect } = item {
        if let ShapeDetails::Rect(d) = &shape.details {
            let svg_path = format!(
                "{} {} {} {}",
                to_svg_path(
                    shape.start.into(),
                    (shape.end.x, shape.start.y),
                    (&shape.options.roughness).into(),
                    2,
                    1.0,
                ),
                to_svg_path(
                    (shape.end.x, shape.start.y),
                    shape.end.into(),
                    (&shape.options.roughness).into(),
                    2,
                    1.0,
                ),
                to_svg_path(
                    shape.end.into(),
                    (shape.start.x, shape.end.y),
                    (&shape.options.roughness).into(),
                    2,
                    1.0,
                ),
                to_svg_path(
                    (shape.start.x, shape.end.y),
                    shape.start.into(),
                    (&shape.options.roughness).into(),
                    2,
                    1.0,
                ),
            );
            path.set_attribute("d", &svg_path)?;
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
            Err(JsValue::from_str("Called update_rect with non-rect shape"))
        }
    } else {
        Err(JsValue::from_str("Called update_rect with non-rect item"))
    }
}
