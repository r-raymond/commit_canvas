mod arrow;
mod rect;
mod text;
mod utils;
use arrow::create_arrow;

use std::collections::HashMap;
use std::error::Error;

use self::{
    arrow::update_arrow,
    rect::update_rect,
    text::{create_text, update_text},
};

use crate::utils::to_error;
use crate::view::ui::rect::create_rect;
use commitcanvas::model::{EventHistory, Guid, ShapeDetails};
use commitcanvas::view::{Event, View};

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
    Text {
        group: web_sys::SvgElement,
        text: web_sys::SvgElement,
        selector: web_sys::SvgElement,
        #[allow(dead_code)]
        selector_closure: wasm_bindgen::closure::Closure<dyn Fn(web_sys::MouseEvent)>,
    },
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
            Item::Text { group, .. } => {
                group.remove();
            }
        }
    }
}

impl View for UIView {
    fn process_event(&mut self, event: Event) -> Result<(), Box<dyn Error + Send + Sync>> {
        match event {
            Event::Reload { shapes } => {
                self.items.clear();
                for (guid, config) in shapes {
                    match config.details {
                        ShapeDetails::Arrow(_) => {
                            log::info!("rendering arrow: {:?}", guid);
                            let item = create_arrow(*guid, config).map_err(to_error)?;
                            self.items.insert(*guid, item);
                        }
                        ShapeDetails::Rect(_) => {
                            log::info!("rendering rect: {:?}", guid);
                            let item = create_rect(*guid, config).map_err(to_error)?;
                            self.items.insert(*guid, item);
                        }
                        ShapeDetails::Text(_) => {
                            log::debug!("rendering text: {:?}", guid);
                            let item = create_text(*guid, config).map_err(to_error)?;
                            self.items.insert(*guid, item);
                        }
                    }
                }
            }
            Event::Modify { event } => match event {
                EventHistory::Add { guid, config } => match config.details {
                    ShapeDetails::Arrow(_) => {
                        log::info!("rendering arrow: {:?}", guid);
                        let item = create_arrow(guid, &config).map_err(to_error)?;
                        self.items.insert(guid, item);
                    }
                    ShapeDetails::Rect(_) => {
                        log::info!("rendering rect: {:?}", guid);
                        let item = create_rect(guid, &config).map_err(to_error)?;
                        self.items.insert(guid, item);
                    }
                    ShapeDetails::Text(_) => {
                        log::debug!("rendering text: {:?}", guid);
                        let item = create_text(guid, &config).map_err(to_error)?;
                        self.items.insert(guid, item);
                    }
                },
                EventHistory::Remove { guid, .. } => {
                    if self.items.remove(&guid).is_some() {
                        log::info!("removing config: {:?}", guid);
                    } else {
                        log::warn!("deleting nonexistent config: {:?}", guid);
                    }
                }
                EventHistory::Modify { guid, to, .. } => match to.details {
                    ShapeDetails::Arrow(_) => {
                        if let Some(item) = self.items.get(&guid) {
                            update_arrow(&to, item).map_err(to_error)?;
                        } else {
                            log::warn!("Updating nonexistent config: {:?}", guid);
                        }
                    }
                    ShapeDetails::Rect(_) => {
                        if let Some(item) = self.items.get(&guid) {
                            update_rect(&to, item).map_err(to_error)?;
                        } else {
                            log::warn!("Updating nonexistent config: {:?}", guid);
                        }
                    }
                    ShapeDetails::Text(_) => {
                        if let Some(item) = self.items.get(&guid) {
                            update_text(&to, item).map_err(to_error)?;
                        } else {
                            log::warn!("Updating nonexistent config: {:?}", guid);
                        }
                    }
                },
                EventHistory::Checkpoint => {}
            },
        };

        Ok(())
    }
}
