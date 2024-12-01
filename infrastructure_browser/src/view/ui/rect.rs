use crate::globals::{CONTROL, DOCUMENT, SVG_VIEW_GROUP};

use super::utils::to_identifier;
use commitcanvas::model::{Guid, ShapeConfig, ShapeDetails};
use commitcanvas::settings::PIXEL_STEP;

use rough::to_svg_path;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use super::Item;

fn render_path(start: (f32, f32), end: (f32, f32), roughness: f32, rounding: f32) -> String {
    let x = start.0.min(end.0);
    let y = start.1.min(end.1);
    let p = start.0.max(end.0);
    let q = start.1.max(end.1);
    let rounding = rounding.min(0.3 * (p - x)).min(0.3 * (q - y));
    let rounding_factor = 0.3;
    format!(
        "{} {} {} {} {} {} {} {}",
        to_svg_path((x + rounding, y), (p - rounding, y), roughness, 2, 1.0,),
        format_args!(
            "M {} {} C {} {} {} {} {} {}",
            p - rounding,
            y,
            p - rounding_factor * rounding,
            y,
            p,
            y + rounding_factor * rounding,
            p,
            y + rounding
        ),
        to_svg_path((p, y + rounding), (p, q - rounding), roughness, 2, 1.0,),
        format_args!(
            "M {} {} C {} {} {} {} {} {}",
            p,
            q - rounding,
            p,
            q - rounding_factor * rounding,
            p - rounding_factor * rounding,
            q,
            p - rounding,
            q
        ),
        to_svg_path((p - rounding, q), (x + rounding, q), roughness, 2, 1.0,),
        format_args!(
            "M {} {} C {} {} {} {} {} {}",
            x + rounding,
            q,
            x + rounding_factor * rounding,
            q,
            x,
            q - rounding_factor * rounding,
            x,
            q - rounding
        ),
        to_svg_path((x, q - rounding), (x, y + rounding), roughness, 2, 1.0,),
        format_args!(
            "M {} {} C {} {} {} {} {} {}",
            x,
            y + rounding,
            x,
            y + rounding_factor * rounding,
            x + rounding_factor * rounding,
            y,
            x + rounding,
            y
        ),
    )
}

pub fn create_rect(guid: Guid, config: &ShapeConfig) -> Result<Item, JsValue> {
    if let ShapeDetails::Rect(d) = &config.details {
        let svg_path = render_path(
            config.start.into(),
            config.end.into(),
            (&config.options.roughness).into(),
            PIXEL_STEP * 2.0,
        );

        let path = DOCUMENT
            .with(|document| {
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
            })?
            .dyn_into::<web_sys::SvgElement>()?;
        path.set_attribute("d", &svg_path)?;
        path.set_attribute("class", "cc_rect")?;
        path.set_attribute("filter", "url(#cc_pencil_texture_4)")?;

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
        group.set_id(&to_identifier(guid));
        group.append_child(&path)?;
        group.append_child(&rect)?;

        let selector = DOCUMENT
            .with(|document| {
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
            })?
            .dyn_into::<web_sys::SvgElement>()?;
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

        Ok(Item::Rect {
            group,
            path,
            rect,
            selector,
            selector_closure,
        })
    } else {
        Err(JsValue::from_str("called create_rect with non-rect config"))
    }
}

pub fn update_rect(config: &ShapeConfig, item: &Item) -> Result<(), JsValue> {
    if let Item::Rect {
        path,
        rect,
        selector,
        ..
    } = item
    {
        if let ShapeDetails::Rect(d) = &config.details {
            let svg_path = render_path(
                config.start.into(),
                config.end.into(),
                (&config.options.roughness).into(),
                PIXEL_STEP * 8.0,
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
            Err(JsValue::from_str("called update_rect with non-rect config"))
        }
    } else {
        Err(JsValue::from_str("Called update_rect with non-rect item"))
    }
}
