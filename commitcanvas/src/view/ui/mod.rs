mod arrow;
use arrow::create_arrow;

use std::collections::HashMap;

use rough::to_svg_path;
use wasm_bindgen::JsValue;

use super::View;
use crate::{
    globals::SVG,
    model::{EventHistory, Guid, ShapeDetails},
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
                            log::info!("Rendering arrow: {:?}", shape.guid);
                            let path = create_arrow(shape)?;
                            SVG.with(|svg| svg.append_child(&path))?;
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
            super::event::Event::Modify { event } => {
                match event {
                    EventHistory::AddShape { shape } => {
                        match shape.details {
                            ShapeDetails::Arrow(_) => {
                                log::info!("Rendering arrow: {:?}", shape.guid);
                                let path = create_arrow(&shape)?;
                                SVG.with(|svg| svg.append_child(&path))?;
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
                    EventHistory::RemoveShape { shape } => {
                        if self.items.remove(&shape.guid).is_some() {
                            log::info!("Removing shape: {:?}", shape.guid);
                        } else {
                            log::warn!("Deleting nonexistent shape: {:?}", shape.guid);
                        }
                    }
                    EventHistory::ModifyShape { from, to } => {
                        match to.details {
                            ShapeDetails::Arrow(_) => {
                                if let Some(Item::Arrow { path }) = self.items.get_mut(&to.guid) {
                                    let svg_path = to_svg_path(
                                        to.start.into(),
                                        to.end.into(),
                                        (&to.options.roughness).into(),
                                        2,
                                        2.0,
                                    );
                                    path.set_attribute("d", &svg_path)?;
                                } else {
                                    log::warn!("Modifying nonexistent shape: {:?}", to.guid);
                                }
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
            }
        };

        Ok(())
    }
}
