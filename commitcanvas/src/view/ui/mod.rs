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
        #[allow(dead_code)]
        selector_closure: wasm_bindgen::closure::Closure<dyn Fn(web_sys::MouseEvent)>,
    },
    Rect {
        group: web_sys::SvgElement,
        path: web_sys::SvgElement,
        rect: web_sys::SvgElement,
        selector: web_sys::SvgElement,
        #[allow(dead_code)]
        selector_closure: wasm_bindgen::closure::Closure<dyn Fn(web_sys::MouseEvent)>,
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
            Item::Rect { group, .. } => {
                group.remove();
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
                for (guid, config) in shapes {
                    match config.details {
                        ShapeDetails::Arrow(_) => {
                            log::info!("rendering arrow: {:?}", guid);
                            let item = create_arrow(*guid, config)?;
                            self.items.insert(*guid, item);
                        }
                        ShapeDetails::Rect(_) => {
                            log::info!("rendering rect: {:?}", guid);
                            let item = create_rect(*guid, config)?;
                            self.items.insert(*guid, item);
                        }
                        ShapeDetails::Text(_) => {
                            // TODO
                        }
                    }
                }
            }
            super::event::Event::Modify { event } => {
                match event {
                    EventHistory::Add { guid, config } => {
                        match config.details {
                            ShapeDetails::Arrow(_) => {
                                log::info!("rendering arrow: {:?}", guid);
                                let item = create_arrow(guid, &config)?;
                                self.items.insert(guid, item);
                            }
                            ShapeDetails::Rect(_) => {
                                log::info!("rendering rect: {:?}", guid);
                                let item = create_rect(guid, &config)?;
                                self.items.insert(guid, item);
                            }
                            ShapeDetails::Text(_) => {
                                // TODO
                            }
                        }
                    }
                    EventHistory::Remove { guid, .. } => {
                        if self.items.remove(&guid).is_some() {
                            log::info!("removing config: {:?}", guid);
                        } else {
                            log::warn!("deleting nonexistent config: {:?}", guid);
                        }
                    }
                    EventHistory::Modify { guid, to, .. } => {
                        match to.details {
                            ShapeDetails::Arrow(_) => {
                                if let Some(item) = self.items.get(&guid) {
                                    update_arrow(&to, item)?;
                                } else {
                                    log::warn!("Updating nonexistent config: {:?}", guid);
                                }
                            }
                            ShapeDetails::Rect(_) => {
                                if let Some(item) = self.items.get(&guid) {
                                    update_rect(&to, item)?;
                                } else {
                                    log::warn!("Updating nonexistent config: {:?}", guid);
                                }
                            }
                            ShapeDetails::Text(_) => {
                                // TODO
                            }
                        }
                    }
                    EventHistory::Checkpoint => {}
                }
            }
        };

        Ok(())
    }
}
