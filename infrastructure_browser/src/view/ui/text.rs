use crate::globals::{CONTROL, DOCUMENT, SVG_VIEW_GROUP};

use super::utils::to_identifier;
use commitcanvas::model::{Guid, ShapeConfig, ShapeDetails};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use super::Item;

pub fn create_text(guid: Guid, config: &ShapeConfig) -> Result<Item, JsValue> {
    if let ShapeDetails::Text(d) = &config.details {
        let x = config.start.x.min(config.end.x);
        let y = config.start.y.min(config.end.y);
        let width = (config.end.x - config.start.x).abs();
        let height = (config.end.y - config.start.y).abs();

        // Create the text element
        let text = DOCUMENT
            .with(|document| {
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "text")
            })?
            .dyn_into::<web_sys::SvgElement>()?;

        text.set_attribute("x", &x.to_string())?;
        text.set_attribute("y", &(y + 16.0).to_string())?; // Offset to position text correctly
        text.set_attribute("class", "cc_text")?;
        text.set_attribute("font-size", (&d.font_size).into())?;
        text.set_attribute("dominant-baseline", "hanging")?;

        // Add content
        let content = d.content.clone();
        if content.is_empty() {
            text.set_text_content(Some("Text"));
        } else {
            text.set_text_content(Some(&content));
        }

        // Create the group and add the text element
        let group = DOCUMENT
            .with(|document| document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g"))?
            .dyn_into::<web_sys::SvgElement>()?;
        group.set_id(&to_identifier(guid));
        group.append_child(&text)?;

        // Create an invisible rect for selection
        let selector = DOCUMENT
            .with(|document| {
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")
            })?
            .dyn_into::<web_sys::SvgElement>()?;
        selector.set_attribute("x", &x.to_string())?;
        selector.set_attribute("y", &y.to_string())?;
        selector.set_attribute("width", &width.to_string())?;
        selector.set_attribute("height", &height.to_string())?;
        selector.set_attribute("class", "cc_selector")?;

        // Add the selector click handler
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

        Ok(Item::Text {
            group,
            text,
            selector,
            selector_closure,
        })
    } else {
        Err(JsValue::from_str("called create_text with non-text config"))
    }
}

pub fn update_text(config: &ShapeConfig, item: &Item) -> Result<(), JsValue> {
    if let Item::Text { text, selector, .. } = item {
        if let ShapeDetails::Text(d) = &config.details {
            let x = config.start.x.min(config.end.x);
            let y = config.start.y.min(config.end.y);
            let width = (config.end.x - config.start.x).abs();
            let height = (config.end.y - config.start.y).abs();

            // Update text position and content
            text.set_attribute("x", &x.to_string())?;
            text.set_attribute("y", &(y + 16.0).to_string())?;
            text.set_attribute("font-size", (&d.font_size).into())?;

            // Update content
            let content = d.content.clone();
            if content.is_empty() {
                text.set_text_content(Some("Text"));
            } else {
                text.set_text_content(Some(&content));
            }

            // Update selector position
            selector.set_attribute("x", &x.to_string())?;
            selector.set_attribute("y", &y.to_string())?;
            selector.set_attribute("width", &width.to_string())?;
            selector.set_attribute("height", &height.to_string())?;

            Ok(())
        } else {
            Err(JsValue::from_str("called update_text with non-text config"))
        }
    } else {
        Err(JsValue::from_str("Called update_text with non-text item"))
    }
}
