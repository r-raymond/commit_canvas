use std::collections::HashMap;

use rough::to_svg_path;
use wasm_bindgen::JsValue;

use super::View;
use crate::{
    globals::DOCUMENT,
    model::{Guid, ShapeDetails},
};

pub struct UIView {
    pub items: HashMap<Guid, Item>,
}

enum Item {
    Arrow { path: web_sys::Element },
    Rect { path: web_sys::Element },
    Text { text: web_sys::Element },
}

impl Drop for Item {
    fn drop(&mut self) {
        match self {
            Item::Arrow { path } => {
                path.remove();
            }
            Item::Rect { path } => {
                path.remove();
            }
            Item::Text { text } => {
                text.remove();
            }
        }
    }
}

impl View for UIView {
    fn process_event(&mut self, event: super::event::Event) -> Result<(), JsValue> {
        match event {
            super::event::Event::Reload { shapes } => {
                self.items.clear();
                for shape in shapes {
                    match shape.details {
                        ShapeDetails::Arrow(_) => {
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
                            self.items.insert(shape.guid, Item::Arrow { path });
                        }
                        ShapeDetails::Rect(_) => {
                            // TODO
                        }
                        ShapeDetails::Text(_) => {
                            // TODO
                        }
                    }
                }
            }
            super::event::Event::Modify { .. } => {
                // TODO
            }
        };

        Ok(())
    }
}
