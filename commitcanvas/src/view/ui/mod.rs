mod arrow;
mod rect;
use arrow::create_arrow;

use std::collections::HashMap;

use wasm_bindgen::JsValue;

use self::{arrow::update_arrow, rect::update_rect};

use super::View;
use crate::{
    model::{EventHistory, Guid, ShapeDetails},
    view::ui::rect::create_rect,
};

pub struct UIView {
    pub items: HashMap<Guid, Item>,
}

#[derive(Debug)]
pub enum Item {
    Arrow {
        group: web_sys::SvgElement,
        path: web_sys::SvgPathElement,
        selector: web_sys::SvgPathElement,
        selector_closure: wasm_bindgen::closure::Closure<dyn Fn(web_sys::MouseEvent)>,
    },
    Rect {
        path: web_sys::Element,
        rect: web_sys::Element,
    },
    #[allow(dead_code)]
    Text { text: web_sys::Element },
}

impl UIView {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl Drop for Item {
    fn drop(&mut self) {
        match self {
            Item::Arrow { group, .. } => {
                group.remove();
            }
            Item::Rect { path, rect } => {
                path.remove();
                rect.remove();
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
                            log::info!("rendering arrow: {:?}", shape.guid);
                            let item = create_arrow(shape)?;
                            self.items.insert(shape.guid, item);
                        }
                        ShapeDetails::Rect(_) => {
                            log::info!("rendering rect: {:?}", shape.guid);
                            let item = create_rect(shape)?;
                            self.items.insert(shape.guid, item);
                        }
                        ShapeDetails::Text(_) => {
                            // TODO
                        }
                    }
                }
            }
            super::event::Event::Modify { event } => {
                match event {
                    EventHistory::Add { shape } => {
                        match shape.details {
                            ShapeDetails::Arrow(_) => {
                                log::info!("rendering arrow: {:?}", shape.guid);
                                let item = create_arrow(&shape)?;
                                self.items.insert(shape.guid, item);
                            }
                            ShapeDetails::Rect(_) => {
                                log::info!("rendering rect: {:?}", shape.guid);
                                let item = create_rect(&shape)?;
                                self.items.insert(shape.guid, item);
                            }
                            ShapeDetails::Text(_) => {
                                // TODO
                            }
                        }
                    }
                    EventHistory::Remove { shape } => {
                        if self.items.remove(&shape.guid).is_some() {
                            log::info!("removing shape: {:?}", shape.guid);
                        } else {
                            log::warn!("deleting nonexistent shape: {:?}", shape.guid);
                        }
                    }
                    EventHistory::Modify { to, .. } => {
                        match to.details {
                            ShapeDetails::Arrow(_) => {
                                if let Some(item) = self.items.get(&to.guid) {
                                    update_arrow(&to, item)?;
                                } else {
                                    log::warn!("Updating nonexistent shape: {:?}", to.guid);
                                }
                            }
                            ShapeDetails::Rect(_) => {
                                if let Some(item) = self.items.get(&to.guid) {
                                    update_rect(&to, item)?;
                                } else {
                                    log::warn!("Updating nonexistent shape: {:?}", to.guid);
                                }
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
